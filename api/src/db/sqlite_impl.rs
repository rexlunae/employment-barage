//! SQLite repository implementations

use super::repository::*;
use super::Database;
use crate::models::*;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::Row;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// SQLite implementation of JobRepository
pub struct SqliteJobRepository {
    db: Database,
}

impl SqliteJobRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl JobRepository for SqliteJobRepository {
    async fn create(&self, job: &Job) -> Result<Job> {
        let requirements_json = serde_json::to_string(&job.requirements)?;
        
        let (salary_min, salary_max, salary_currency, salary_period) = match &job.salary_range {
            Some(range) => (
                Some(range.min as i64),
                Some(range.max as i64),
                Some(range.currency.clone()),
                Some(format!("{:?}", range.period)),
            ),
            None => (None, None, None, None),
        };
        
        let source_str = match &job.source {
            JobSource::LinkedIn => "LinkedIn",
            JobSource::Indeed => "Indeed",
            JobSource::Glassdoor => "Glassdoor",
            JobSource::AngelList => "AngelList",
            JobSource::Other(s) => s,
        };
        
        sqlx::query(
            r#"
            INSERT INTO jobs (id, title, company, location, description, requirements, 
                salary_min, salary_max, salary_currency, salary_period, source, source_url, 
                posted_date, scraped_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(job.id.to_string())
        .bind(&job.title)
        .bind(&job.company)
        .bind(&job.location)
        .bind(&job.description)
        .bind(&requirements_json)
        .bind(salary_min)
        .bind(salary_max)
        .bind(salary_currency)
        .bind(salary_period)
        .bind(source_str)
        .bind(&job.source_url)
        .bind(job.posted_date.to_rfc3339())
        .bind(job.scraped_at.to_rfc3339())
        .execute(self.db.pool())
        .await?;
        
        Ok(job.clone())
    }
    
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Job>> {
        let row = sqlx::query(
            "SELECT * FROM jobs WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(self.db.pool())
        .await?;
        
        match row {
            Some(row) => Ok(Some(row_to_job(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn search(&self, query: &JobSearchQuery) -> Result<Vec<Job>> {
        // Build the query with all parameters owned
        let keywords_pattern = query.keywords.as_ref().map(|kw| format!("%{}%", kw));
        let location_pattern = query.location.as_ref().map(|loc| format!("%{}%", loc));
        
        let source_strings: Vec<String> = query.sources.iter().map(|s| {
            match s {
                JobSource::LinkedIn => "LinkedIn".to_string(),
                JobSource::Indeed => "Indeed".to_string(),
                JobSource::Glassdoor => "Glassdoor".to_string(),
                JobSource::AngelList => "AngelList".to_string(),
                JobSource::Other(name) => name.clone(),
            }
        }).collect();
        
        let mut sql = String::from("SELECT * FROM jobs WHERE 1=1");
        
        if keywords_pattern.is_some() {
            sql.push_str(" AND (title LIKE ?1 OR description LIKE ?1 OR company LIKE ?1)");
        }
        if location_pattern.is_some() {
            sql.push_str(" AND location LIKE ?2");
        }
        if query.min_salary.is_some() {
            sql.push_str(" AND salary_min >= ?3");
        }
        if query.remote_only {
            sql.push_str(" AND is_remote = 1");
        }
        if !source_strings.is_empty() {
            let placeholders: Vec<String> = (0..source_strings.len())
                .map(|i| format!("?{}", i + 4))
                .collect();
            sql.push_str(&format!(" AND source IN ({})", placeholders.join(",")));
        }
        
        sql.push_str(" ORDER BY posted_date DESC");
        
        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = query.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        
        // For SQLite, we need to use a simpler approach with raw queries
        // due to the dynamic nature of the query
        let rows = sqlx::query(&sql)
            .bind(keywords_pattern.as_deref().unwrap_or(""))
            .bind(location_pattern.as_deref().unwrap_or(""))
            .bind(query.min_salary.unwrap_or(0) as i64)
            .fetch_all(self.db.pool())
            .await?;
        
        let mut jobs = Vec::new();
        for row in rows {
            jobs.push(row_to_job(&row)?);
        }
        
        Ok(jobs)
    }
    
    async fn get_saved(&self) -> Result<Vec<Job>> {
        let rows = sqlx::query("SELECT * FROM jobs WHERE is_saved = 1 ORDER BY created_at DESC")
            .fetch_all(self.db.pool())
            .await?;
        
        let mut jobs = Vec::new();
        for row in rows {
            jobs.push(row_to_job(&row)?);
        }
        Ok(jobs)
    }
    
    async fn save(&self, id: &Uuid) -> Result<()> {
        sqlx::query("UPDATE jobs SET is_saved = 1, updated_at = datetime('now') WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
    
    async fn unsave(&self, id: &Uuid) -> Result<()> {
        sqlx::query("UPDATE jobs SET is_saved = 0, updated_at = datetime('now') WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
    
    async fn update_match_score(&self, id: &Uuid, score: f64) -> Result<()> {
        sqlx::query("UPDATE jobs SET match_score = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(score)
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
    
    async fn upsert_by_source(&self, job: &Job) -> Result<Job> {
        // Check if job already exists by source URL
        let existing = sqlx::query("SELECT id FROM jobs WHERE source_url = ?")
            .bind(&job.source_url)
            .fetch_optional(self.db.pool())
            .await?;
        
        if existing.is_some() {
            // Update existing
            let requirements_json = serde_json::to_string(&job.requirements)?;
            sqlx::query(
                "UPDATE jobs SET title = ?, company = ?, location = ?, description = ?, 
                 requirements = ?, updated_at = datetime('now') WHERE source_url = ?"
            )
            .bind(&job.title)
            .bind(&job.company)
            .bind(&job.location)
            .bind(&job.description)
            .bind(&requirements_json)
            .bind(&job.source_url)
            .execute(self.db.pool())
            .await?;
            
            Ok(job.clone())
        } else {
            self.create(job).await
        }
    }
}

/// Helper to convert a database row to a Job struct
fn row_to_job(row: &sqlx::sqlite::SqliteRow) -> Result<Job> {
    let id_str: String = row.get("id");
    let requirements_json: String = row.get("requirements");
    let source_str: String = row.get("source");
    let posted_date_str: String = row.get("posted_date");
    let scraped_at_str: String = row.get("scraped_at");
    
    let salary_min: Option<i64> = row.get("salary_min");
    let salary_max: Option<i64> = row.get("salary_max");
    let salary_currency: Option<String> = row.get("salary_currency");
    let salary_period: Option<String> = row.get("salary_period");
    
    let salary_range = match (salary_min, salary_max) {
        (Some(min), Some(max)) => Some(SalaryRange {
            min: min as u32,
            max: max as u32,
            currency: salary_currency.unwrap_or_else(|| "USD".to_string()),
            period: match salary_period.as_deref() {
                Some("Hourly") => SalaryPeriod::Hourly,
                _ => SalaryPeriod::Annual,
            },
        }),
        _ => None,
    };
    
    let source = match source_str.as_str() {
        "LinkedIn" => JobSource::LinkedIn,
        "Indeed" => JobSource::Indeed,
        "Glassdoor" => JobSource::Glassdoor,
        "AngelList" => JobSource::AngelList,
        other => JobSource::Other(other.to_string()),
    };
    
    Ok(Job {
        id: Uuid::parse_str(&id_str)?,
        title: row.get("title"),
        company: row.get("company"),
        location: row.get("location"),
        description: row.get("description"),
        requirements: serde_json::from_str(&requirements_json)?,
        salary_range,
        source,
        source_url: row.get("source_url"),
        posted_date: DateTime::parse_from_rfc3339(&posted_date_str)?.with_timezone(&Utc),
        scraped_at: DateTime::parse_from_rfc3339(&scraped_at_str)?.with_timezone(&Utc),
    })
}
