//! Job source implementations for fetching jobs from various platforms
//!
//! This module provides scrapers/API clients for various job sources.
//! Each source implements the `JobSourceProvider` trait.

pub mod remotive;
pub mod hn_who_is_hiring;
pub mod arbeitnow;

use crate::models::Job;
use anyhow::Result;
use async_trait::async_trait;

/// Trait for job source providers
#[async_trait]
pub trait JobSourceProvider: Send + Sync {
    /// Fetch jobs from this source
    /// 
    /// # Arguments
    /// * `keywords` - Optional search keywords
    /// * `location` - Optional location filter
    /// * `limit` - Maximum number of jobs to fetch
    async fn fetch_jobs(
        &self,
        keywords: Option<&str>,
        location: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Job>>;
    
    /// Get the source name
    fn source_name(&self) -> &'static str;
}

/// Aggregate job fetcher that pulls from multiple sources
pub struct JobAggregator {
    sources: Vec<Box<dyn JobSourceProvider>>,
}

impl JobAggregator {
    /// Create a new job aggregator with default sources
    pub fn new() -> Self {
        let sources: Vec<Box<dyn JobSourceProvider>> = vec![
            Box::new(remotive::RemotiveClient::new()),
            Box::new(hn_who_is_hiring::HNWhoIsHiringClient::new()),
            Box::new(arbeitnow::ArbeitnowClient::new()),
        ];
        
        Self { sources }
    }
    
    /// Fetch jobs from all sources
    pub async fn fetch_all(
        &self,
        keywords: Option<&str>,
        location: Option<&str>,
        limit_per_source: Option<u32>,
    ) -> Result<Vec<Job>> {
        let mut all_jobs = Vec::new();
        
        for source in &self.sources {
            match source.fetch_jobs(keywords, location, limit_per_source).await {
                Ok(jobs) => {
                    tracing::info!("Fetched {} jobs from {}", jobs.len(), source.source_name());
                    all_jobs.extend(jobs);
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch from {}: {}", source.source_name(), e);
                }
            }
        }
        
        Ok(all_jobs)
    }
}

impl Default for JobAggregator {
    fn default() -> Self {
        Self::new()
    }
}
