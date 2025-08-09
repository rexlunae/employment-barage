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
    Other(String),
}