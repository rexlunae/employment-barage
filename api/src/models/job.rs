use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub id: Uuid,
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub salary_range: Option<SalaryRange>,
    pub source: JobSource,
    pub source_url: String,
    pub posted_date: DateTime<Utc>,
    pub scraped_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalaryRange {
    pub min: u32,
    pub max: u32,
    pub currency: String,
    pub period: SalaryPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SalaryPeriod {
    Hourly,
    Annual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobSource {
    LinkedIn,
    Indeed,
    Glassdoor,
    AngelList,
    /// Remote jobs from Remotive.com (free API, no key required)
    Remotive,
    /// Jobs from HN "Who's Hiring?" threads (free API, no key required)
    HNWhoIsHiring,
    /// Jobs from Arbeitnow.com - Germany/EU focused (free API, no key required)
    Arbeitnow,
    /// Other/custom source
    Other(String),
}

impl JobSource {
    /// Get the display name for this source
    pub fn display_name(&self) -> &str {
        match self {
            JobSource::LinkedIn => "LinkedIn",
            JobSource::Indeed => "Indeed",
            JobSource::Glassdoor => "Glassdoor",
            JobSource::AngelList => "AngelList",
            JobSource::Remotive => "Remotive",
            JobSource::HNWhoIsHiring => "HN Who's Hiring",
            JobSource::Arbeitnow => "Arbeitnow",
            JobSource::Other(name) => name,
        }
    }
    
    /// Check if this source requires an API key
    pub fn requires_api_key(&self) -> bool {
        match self {
            JobSource::LinkedIn => true,
            JobSource::Indeed => true,
            JobSource::Glassdoor => true,
            JobSource::AngelList => true,
            JobSource::Remotive => false,
            JobSource::HNWhoIsHiring => false,
            JobSource::Arbeitnow => false,
            JobSource::Other(_) => false,
        }
    }
}