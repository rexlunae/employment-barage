//! Remotive API client for fetching remote jobs
//!
//! Remotive provides a free API for remote job listings.
//! API Documentation: https://remotive.com/api-documentation
//! No API key required (rate limited to ~4 requests/day recommended)

use super::JobSourceProvider;
use crate::models::{Job, JobSource, SalaryRange, SalaryPeriod};
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

const REMOTIVE_API_URL: &str = "https://remotive.com/api/remote-jobs";

/// Client for the Remotive job board API
pub struct RemotiveClient {
    client: reqwest::Client,
}

impl RemotiveClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for RemotiveClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct RemotiveResponse {
    #[serde(rename = "job-count")]
    job_count: Option<u32>,
    jobs: Vec<RemotiveJob>,
}

#[derive(Debug, Deserialize)]
struct RemotiveJob {
    id: u64,
    url: String,
    title: String,
    company_name: String,
    #[serde(default)]
    company_logo: Option<String>,
    category: String,
    #[serde(default)]
    tags: Vec<String>,
    job_type: String,
    publication_date: String,
    candidate_required_location: String,
    #[serde(default)]
    salary: Option<String>,
    description: String,
}

#[async_trait]
impl JobSourceProvider for RemotiveClient {
    async fn fetch_jobs(
        &self,
        keywords: Option<&str>,
        _location: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Job>> {
        let mut url = reqwest::Url::parse(REMOTIVE_API_URL)?;
        
        // Remotive supports category and search filters
        if let Some(kw) = keywords {
            url.query_pairs_mut().append_pair("search", kw);
        }
        
        if let Some(lim) = limit {
            url.query_pairs_mut().append_pair("limit", &lim.to_string());
        }
        
        let response: RemotiveResponse = self.client
            .get(url)
            .header("User-Agent", "employment-barage/1.0")
            .send()
            .await
            .context("Failed to fetch from Remotive API")?
            .json()
            .await
            .context("Failed to parse Remotive response")?;
        
        let jobs = response.jobs
            .into_iter()
            .map(|rj| convert_remotive_job(rj))
            .collect();
        
        Ok(jobs)
    }
    
    fn source_name(&self) -> &'static str {
        "Remotive"
    }
}

fn convert_remotive_job(rj: RemotiveJob) -> Job {
    let posted_date = DateTime::parse_from_rfc3339(&rj.publication_date)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    
    // Parse salary if available (format varies, e.g., "$50k-$80k", "$30-$35/hour")
    let salary_range = rj.salary.as_ref().and_then(|s| parse_salary(s));
    
    // Convert tags to requirements
    let requirements = rj.tags.clone();
    
    Job {
        id: Uuid::new_v4(),
        title: rj.title,
        company: rj.company_name,
        location: if rj.candidate_required_location.is_empty() {
            "Remote".to_string()
        } else {
            format!("Remote - {}", rj.candidate_required_location)
        },
        description: clean_html(&rj.description),
        requirements,
        salary_range,
        source: JobSource::Other("Remotive".to_string()),
        source_url: rj.url,
        posted_date,
        scraped_at: Utc::now(),
    }
}

/// Clean HTML tags from description
fn clean_html(html: &str) -> String {
    // Simple HTML tag removal - a proper implementation would use a library
    let mut result = html.to_string();
    
    // Replace common HTML entities
    result = result.replace("&amp;", "&");
    result = result.replace("&lt;", "<");
    result = result.replace("&gt;", ">");
    result = result.replace("&nbsp;", " ");
    result = result.replace("&quot;", "\"");
    result = result.replace("&#x27;", "'");
    result = result.replace("&#39;", "'");
    result = result.replace("&#x2F;", "/");
    
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
    let tag_re = regex::Regex::new(r"<[^>]+>").unwrap();
    result = tag_re.replace_all(&result, "").to_string();
    
    // Clean up whitespace
    let ws_re = regex::Regex::new(r"\n{3,}").unwrap();
    result = ws_re.replace_all(&result, "\n\n").to_string();
    
    result.trim().to_string()
}

/// Parse salary string into SalaryRange
fn parse_salary(salary_str: &str) -> Option<SalaryRange> {
    let salary = salary_str.to_lowercase();
    
    // Check if hourly
    let is_hourly = salary.contains("/hour") || salary.contains("per hour") || salary.contains("/hr");
    
    // Extract numbers using regex
    let num_re = regex::Regex::new(r"\$?(\d+(?:,\d{3})*(?:\.\d+)?)\s*k?").unwrap();
    let numbers: Vec<u32> = num_re
        .captures_iter(&salary)
        .filter_map(|cap| {
            let num_str = cap.get(1)?.as_str().replace(",", "");
            let mut num: f64 = num_str.parse().ok()?;
            
            // Check if this match includes 'k' for thousands
            if cap.get(0)?.as_str().to_lowercase().ends_with('k') {
                num *= 1000.0;
            }
            
            Some(num as u32)
        })
        .collect();
    
    if numbers.len() >= 2 {
        Some(SalaryRange {
            min: numbers[0],
            max: numbers[1],
            currency: "USD".to_string(),
            period: if is_hourly { SalaryPeriod::Hourly } else { SalaryPeriod::Annual },
        })
    } else if numbers.len() == 1 {
        Some(SalaryRange {
            min: numbers[0],
            max: numbers[0],
            currency: "USD".to_string(),
            period: if is_hourly { SalaryPeriod::Hourly } else { SalaryPeriod::Annual },
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_salary_range() {
        let salary = parse_salary("$50k-$80k").unwrap();
        assert_eq!(salary.min, 50000);
        assert_eq!(salary.max, 80000);
        assert_eq!(salary.period, SalaryPeriod::Annual);
    }
    
    #[test]
    fn test_parse_salary_hourly() {
        let salary = parse_salary("$30-$35/hour").unwrap();
        assert_eq!(salary.min, 30);
        assert_eq!(salary.max, 35);
        assert_eq!(salary.period, SalaryPeriod::Hourly);
    }
}
