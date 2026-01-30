//! Arbeitnow API client for fetching jobs
//!
//! Arbeitnow is a job board focused on Germany and Europe.
//! API is free and requires no authentication.
//! API URL: https://www.arbeitnow.com/api/job-board-api

use super::JobSourceProvider;
use crate::models::{Job, JobSource};
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{Utc, TimeZone};
use serde::Deserialize;
use uuid::Uuid;
use regex::Regex;

const ARBEITNOW_API_URL: &str = "https://www.arbeitnow.com/api/job-board-api";

/// Client for the Arbeitnow job board API
pub struct ArbeitnowClient {
    client: reqwest::Client,
}

impl ArbeitnowClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for ArbeitnowClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct ArbeitnowResponse {
    data: Vec<ArbeitnowJob>,
}

#[derive(Debug, Deserialize)]
struct ArbeitnowJob {
    slug: String,
    company_name: String,
    title: String,
    description: String,
    remote: bool,
    url: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    job_types: Vec<String>,
    location: String,
    created_at: i64,
}

#[async_trait]
impl JobSourceProvider for ArbeitnowClient {
    async fn fetch_jobs(
        &self,
        keywords: Option<&str>,
        location: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Job>> {
        let response: ArbeitnowResponse = self.client
            .get(ARBEITNOW_API_URL)
            .header("User-Agent", "employment-barage/1.0")
            .send()
            .await
            .context("Failed to fetch from Arbeitnow API")?
            .json()
            .await
            .context("Failed to parse Arbeitnow response")?;
        
        let limit = limit.unwrap_or(50) as usize;
        
        let jobs: Vec<Job> = response.data
            .into_iter()
            .filter(|aj| {
                // Apply keyword filter if specified
                let matches_keywords = keywords.map(|kw| {
                    let kw_lower = kw.to_lowercase();
                    aj.title.to_lowercase().contains(&kw_lower)
                        || aj.description.to_lowercase().contains(&kw_lower)
                        || aj.company_name.to_lowercase().contains(&kw_lower)
                        || aj.tags.iter().any(|t| t.to_lowercase().contains(&kw_lower))
                }).unwrap_or(true);
                
                // Apply location filter if specified
                let matches_location = location.map(|loc| {
                    let loc_lower = loc.to_lowercase();
                    aj.location.to_lowercase().contains(&loc_lower)
                        || (loc_lower == "remote" && aj.remote)
                }).unwrap_or(true);
                
                matches_keywords && matches_location
            })
            .take(limit)
            .map(|aj| convert_arbeitnow_job(aj))
            .collect();
        
        Ok(jobs)
    }
    
    fn source_name(&self) -> &'static str {
        "Arbeitnow"
    }
}

fn convert_arbeitnow_job(aj: ArbeitnowJob) -> Job {
    let posted_date = Utc.timestamp_opt(aj.created_at, 0)
        .single()
        .unwrap_or_else(Utc::now);
    
    let location = if aj.remote {
        format!("{} (Remote)", aj.location)
    } else {
        aj.location
    };
    
    // Clean HTML from description
    let description = clean_html(&aj.description);
    
    // Combine tags and job_types as requirements
    let mut requirements = aj.tags;
    requirements.extend(aj.job_types);
    
    Job {
        id: Uuid::new_v4(),
        title: aj.title,
        company: aj.company_name,
        location,
        description,
        requirements,
        salary_range: None, // Arbeitnow doesn't typically include salary
        source: JobSource::Other("Arbeitnow".to_string()),
        source_url: aj.url,
        posted_date,
        scraped_at: Utc::now(),
    }
}

/// Clean HTML tags from description
fn clean_html(html: &str) -> String {
    let mut result = html.to_string();
    
    // Replace common HTML entities
    result = result.replace("&amp;", "&");
    result = result.replace("&lt;", "<");
    result = result.replace("&gt;", ">");
    result = result.replace("&nbsp;", " ");
    result = result.replace("&quot;", "\"");
    result = result.replace("&#x27;", "'");
    result = result.replace("&#39;", "'");
    result = result.replace("&#x26;", "&");
    
    // Replace headings with text + newlines
    let h_re = Regex::new(r"<h[1-6][^>]*>([^<]*)</h[1-6]>").unwrap();
    result = h_re.replace_all(&result, "\n$1\n").to_string();
    
    // Replace block elements with newlines
    result = result.replace("<br>", "\n");
    result = result.replace("<br/>", "\n");
    result = result.replace("<br />", "\n");
    result = result.replace("</p>", "\n\n");
    result = result.replace("</div>", "\n");
    result = result.replace("</li>", "\n");
    result = result.replace("<li>", "• ");
    result = result.replace("<li style=\"\">", "• ");
    
    // Remove all remaining HTML tags
    let tag_re = Regex::new(r"<[^>]+>").unwrap();
    result = tag_re.replace_all(&result, "").to_string();
    
    // Clean up whitespace
    let ws_re = Regex::new(r"\n{3,}").unwrap();
    result = ws_re.replace_all(&result, "\n\n").to_string();
    
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clean_html() {
        let html = "<p>Hello <strong>world</strong></p><ul><li>Item 1</li><li>Item 2</li></ul>";
        let clean = clean_html(html);
        assert!(clean.contains("Hello"));
        assert!(clean.contains("Item 1"));
        assert!(!clean.contains("<"));
    }
}
