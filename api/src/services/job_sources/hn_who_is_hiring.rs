//! Hacker News "Who's Hiring?" thread scraper
//!
//! This module fetches job postings from the monthly "Ask HN: Who is hiring?" threads.
//! Uses the official Hacker News Firebase API - no API key required.
//! 
//! The thread is posted on the first weekday of each month by @whoishiring.

use super::JobSourceProvider;
use crate::models::{Job, JobSource};
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{Utc, TimeZone, Datelike};
use serde::Deserialize;
use uuid::Uuid;
use regex::Regex;

const HN_API_BASE: &str = "https://hacker-news.firebaseio.com/v0";
const WHOISHIRING_USER: &str = "whoishiring";

/// Client for fetching jobs from HN Who's Hiring threads
pub struct HNWhoIsHiringClient {
    client: reqwest::Client,
}

impl HNWhoIsHiringClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    
    /// Get the latest "Who is hiring?" thread ID
    async fn get_latest_hiring_thread_id(&self) -> Result<u64> {
        let url = format!("{}/user/{}.json", HN_API_BASE, WHOISHIRING_USER);
        let user: HNUser = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .context("Failed to fetch whoishiring user")?;
        
        // The user's submissions include all threads
        // "Who is hiring?" threads alternate with "Who wants to be hired?" and "Freelancer?"
        // We need to find the most recent "Who is hiring?" thread
        for &item_id in user.submitted.iter().take(10) {
            if let Ok(item) = self.get_item(item_id).await {
                if let Some(title) = &item.title {
                    if title.contains("Who is hiring?") {
                        return Ok(item_id);
                    }
                }
            }
        }
        
        // Fallback to first submission if we can't find a hiring thread
        user.submitted.first()
            .copied()
            .ok_or_else(|| anyhow::anyhow!("No threads found"))
    }
    
    /// Fetch a single HN item
    async fn get_item(&self, id: u64) -> Result<HNItem> {
        let url = format!("{}/item/{}.json", HN_API_BASE, id);
        self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .context("Failed to fetch HN item")
    }
}

impl Default for HNWhoIsHiringClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct HNUser {
    id: String,
    submitted: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct HNItem {
    id: u64,
    #[serde(default)]
    by: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    time: Option<i64>,
    #[serde(rename = "type")]
    item_type: Option<String>,
    #[serde(default)]
    kids: Vec<u64>,
    #[serde(default)]
    deleted: Option<bool>,
    #[serde(default)]
    dead: Option<bool>,
}

#[async_trait]
impl JobSourceProvider for HNWhoIsHiringClient {
    async fn fetch_jobs(
        &self,
        keywords: Option<&str>,
        location: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Job>> {
        // Get the latest hiring thread
        let thread_id = self.get_latest_hiring_thread_id().await?;
        let thread = self.get_item(thread_id).await?;
        
        let limit = limit.unwrap_or(100) as usize;
        let mut jobs = Vec::new();
        
        // Fetch comments (job postings) from the thread
        // We limit concurrent requests to be nice to the API
        let comment_ids: Vec<u64> = thread.kids.into_iter().take(limit * 2).collect();
        
        for chunk in comment_ids.chunks(10) {
            let futures: Vec<_> = chunk.iter().map(|&id| self.get_item(id)).collect();
            let results = futures::future::join_all(futures).await;
            
            for result in results {
                if let Ok(item) = result {
                    // Skip deleted or dead comments
                    if item.deleted.unwrap_or(false) || item.dead.unwrap_or(false) {
                        continue;
                    }
                    
                    if let Some(_text) = &item.text {
                        // Parse the comment into a job posting
                        if let Some(job) = parse_hn_job_posting(&item, thread_id) {
                            // Apply filters
                            let matches_keywords = keywords.map(|kw| {
                                let kw_lower = kw.to_lowercase();
                                job.title.to_lowercase().contains(&kw_lower)
                                    || job.description.to_lowercase().contains(&kw_lower)
                                    || job.company.to_lowercase().contains(&kw_lower)
                            }).unwrap_or(true);
                            
                            let matches_location = location.map(|loc| {
                                let loc_lower = loc.to_lowercase();
                                job.location.to_lowercase().contains(&loc_lower)
                                    || (loc_lower == "remote" && 
                                        job.location.to_lowercase().contains("remote"))
                            }).unwrap_or(true);
                            
                            if matches_keywords && matches_location {
                                jobs.push(job);
                            }
                        }
                    }
                }
                
                if jobs.len() >= limit {
                    break;
                }
            }
            
            if jobs.len() >= limit {
                break;
            }
        }
        
        Ok(jobs)
    }
    
    fn source_name(&self) -> &'static str {
        "HN Who's Hiring"
    }
}

/// Parse an HN comment into a Job struct
fn parse_hn_job_posting(item: &HNItem, _thread_id: u64) -> Option<Job> {
    let text = item.text.as_ref()?;
    let _author = item.by.as_ref().cloned().unwrap_or_else(|| "Unknown".to_string());
    
    // Clean HTML from the text
    let clean_text = clean_hn_html(text);
    
    // Try to extract company and title from the first line
    // Common formats:
    // "Company Name | Job Title | Location | REMOTE"
    // "Company Name is hiring..."
    let first_line = clean_text.lines().next().unwrap_or("");
    let (company, title, location) = parse_first_line(first_line);
    
    // Build the HN URL for this comment
    let source_url = format!("https://news.ycombinator.com/item?id={}", item.id);
    
    let posted_date = item.time
        .map(|t| Utc.timestamp_opt(t, 0).single())
        .flatten()
        .unwrap_or_else(Utc::now);
    
    Some(Job {
        id: Uuid::new_v4(),
        title,
        company,
        location,
        description: clean_text.clone(),
        requirements: extract_technologies(&clean_text),
        salary_range: None, // HN posts rarely have structured salary info
        source: JobSource::Other("HN Who's Hiring".to_string()),
        source_url,
        posted_date,
        scraped_at: Utc::now(),
    })
}

/// Parse the first line of an HN job post to extract company, title, and location
fn parse_first_line(line: &str) -> (String, String, String) {
    // Try pipe-separated format first: "Company | Title | Location"
    let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
    
    if parts.len() >= 2 {
        let company = parts[0].to_string();
        let title = parts[1].to_string();
        let location = if parts.len() >= 3 {
            parts[2..].join(" | ")
        } else {
            "Unknown".to_string()
        };
        return (company, title, location);
    }
    
    // Try to extract "Company is hiring..." pattern
    let hiring_re = Regex::new(r"^(.+?)\s+(?:is hiring|hiring|looking for|seeks?)\s+(.+)").ok();
    if let Some(re) = hiring_re {
        if let Some(caps) = re.captures(line) {
            let company = caps.get(1).map(|m| m.as_str()).unwrap_or("Unknown").to_string();
            let title = caps.get(2).map(|m| m.as_str()).unwrap_or("Software Engineer").to_string();
            return (company, title, detect_location(line));
        }
    }
    
    // Fallback: use first line as company, generic title
    let company = line.split_whitespace().take(5).collect::<Vec<_>>().join(" ");
    let company = if company.is_empty() { "Unknown".to_string() } else { company };
    
    (company, "Software Engineer".to_string(), detect_location(line))
}

/// Try to detect location from text
fn detect_location(text: &str) -> String {
    let text_upper = text.to_uppercase();
    
    if text_upper.contains("REMOTE") {
        if text_upper.contains("US") || text_upper.contains("USA") {
            return "Remote (US)".to_string();
        }
        if text_upper.contains("EU") || text_upper.contains("EUROPE") {
            return "Remote (EU)".to_string();
        }
        return "Remote".to_string();
    }
    
    if text_upper.contains("ONSITE") || text_upper.contains("ON-SITE") {
        // Try to find city names
        let cities = ["San Francisco", "New York", "Seattle", "Austin", "Boston", 
                      "Chicago", "Los Angeles", "Denver", "Portland", "Miami",
                      "London", "Berlin", "Amsterdam", "Paris", "Toronto"];
        for city in cities {
            if text.contains(city) {
                return format!("{} (Onsite)", city);
            }
        }
        return "Onsite".to_string();
    }
    
    "Unknown".to_string()
}

/// Extract technology keywords from job description
fn extract_technologies(text: &str) -> Vec<String> {
    let techs = [
        "Python", "JavaScript", "TypeScript", "Rust", "Go", "Java", "C++", "C#",
        "Ruby", "PHP", "Scala", "Kotlin", "Swift", "React", "Vue", "Angular",
        "Node.js", "Django", "Flask", "Rails", "Spring", "FastAPI",
        "PostgreSQL", "MySQL", "MongoDB", "Redis", "Elasticsearch",
        "Docker", "Kubernetes", "AWS", "GCP", "Azure", "Terraform",
        "GraphQL", "REST", "gRPC", "Kafka", "RabbitMQ",
        "Machine Learning", "ML", "AI", "Deep Learning", "NLP",
        "iOS", "Android", "React Native", "Flutter",
    ];
    
    let text_lower = text.to_lowercase();
    techs.iter()
        .filter(|&tech| text_lower.contains(&tech.to_lowercase()))
        .map(|&s| s.to_string())
        .collect()
}

/// Clean HN HTML entities and formatting
fn clean_hn_html(html: &str) -> String {
    let mut result = html.to_string();
    
    // Replace HTML entities
    result = result.replace("&#x27;", "'");
    result = result.replace("&#x2F;", "/");
    result = result.replace("&amp;", "&");
    result = result.replace("&lt;", "<");
    result = result.replace("&gt;", ">");
    result = result.replace("&quot;", "\"");
    result = result.replace("&#39;", "'");
    
    // Replace paragraph tags
    result = result.replace("<p>", "\n\n");
    result = result.replace("</p>", "");
    
    // Handle links - extract href
    let link_re = Regex::new(r#"<a[^>]*href="([^"]*)"[^>]*>([^<]*)</a>"#).unwrap();
    result = link_re.replace_all(&result, "$2 ($1)").to_string();
    
    // Remove other HTML tags
    let tag_re = Regex::new(r"<[^>]+>").unwrap();
    result = tag_re.replace_all(&result, "").to_string();
    
    // Clean up extra whitespace
    let ws_re = Regex::new(r"\n{3,}").unwrap();
    result = ws_re.replace_all(&result, "\n\n").to_string();
    
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_first_line_pipe_format() {
        let (company, title, location) = parse_first_line("Acme Corp | Senior Engineer | San Francisco | REMOTE");
        assert_eq!(company, "Acme Corp");
        assert_eq!(title, "Senior Engineer");
        assert!(location.contains("San Francisco"));
    }
    
    #[test]
    fn test_detect_location_remote() {
        assert_eq!(detect_location("We are REMOTE first"), "Remote");
        assert_eq!(detect_location("REMOTE (US only)"), "Remote (US)");
    }
    
    #[test]
    fn test_extract_technologies() {
        let text = "Looking for Python and React developers with AWS experience";
        let techs = extract_technologies(text);
        assert!(techs.contains(&"Python".to_string()));
        assert!(techs.contains(&"React".to_string()));
        assert!(techs.contains(&"AWS".to_string()));
    }
}
