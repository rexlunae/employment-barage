//! SQLite repository implementations

use super::repository::*;
use super::Database;
use crate::models::*;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::Row;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// SQLite implementation of ProfileRepository
pub struct SqliteProfileRepository {
    db: Database,
}

impl SqliteProfileRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProfileRepository for SqliteProfileRepository {
    async fn create(&self, profile: &Profile) -> Result<Profile> {
        sqlx::query(
            r#"
            INSERT INTO profiles (id, user_id, name, email, headline, summary, location, phone, 
                linkedin_url, github_url, portfolio_url, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(profile.id.to_string())
        .bind(profile.user_id.to_string())
        .bind(&profile.name)
        .bind(&profile.email)
        .bind(&profile.headline)
        .bind(&profile.summary)
        .bind(&profile.location)
        .bind(&profile.phone)
        .bind(&profile.linkedin_url)
        .bind(&profile.github_url)
        .bind(&profile.portfolio_url)
        .bind(profile.created_at.to_rfc3339())
        .bind(profile.updated_at.to_rfc3339())
        .execute(self.db.pool())
        .await?;
        
        Ok(profile.clone())
    }
    
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Profile>> {
        let row = sqlx::query("SELECT * FROM profiles WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(self.db.pool())
            .await?;
        
        match row {
            Some(row) => Ok(Some(row_to_profile(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn get_by_user_id(&self, user_id: &Uuid) -> Result<Option<Profile>> {
        let row = sqlx::query("SELECT * FROM profiles WHERE user_id = ?")
            .bind(user_id.to_string())
            .fetch_optional(self.db.pool())
            .await?;
        
        match row {
            Some(row) => Ok(Some(row_to_profile(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn update(&self, profile: &Profile) -> Result<Profile> {
        sqlx::query(
            r#"
            UPDATE profiles SET 
                name = ?, email = ?, headline = ?, summary = ?, location = ?, phone = ?,
                linkedin_url = ?, github_url = ?, portfolio_url = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&profile.name)
        .bind(&profile.email)
        .bind(&profile.headline)
        .bind(&profile.summary)
        .bind(&profile.location)
        .bind(&profile.phone)
        .bind(&profile.linkedin_url)
        .bind(&profile.github_url)
        .bind(&profile.portfolio_url)
        .bind(Utc::now().to_rfc3339())
        .bind(profile.id.to_string())
        .execute(self.db.pool())
        .await?;
        
        Ok(profile.clone())
    }
    
    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM profiles WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
}

/// Helper to convert a database row to a Profile struct
fn row_to_profile(row: &sqlx::sqlite::SqliteRow) -> Result<Profile> {
    let id_str: String = row.get("id");
    let user_id_str: String = row.get("user_id");
    let created_at_str: String = row.get("created_at");
    let updated_at_str: String = row.get("updated_at");
    
    Ok(Profile {
        id: Uuid::parse_str(&id_str)?,
        user_id: Uuid::parse_str(&user_id_str)?,
        name: row.get::<Option<String>, _>("name").unwrap_or_default(),
        email: row.get::<Option<String>, _>("email").unwrap_or_default(),
        headline: row.get("headline"),
        summary: row.get("summary"),
        location: row.get("location"),
        phone: row.get("phone"),
        linkedin_url: row.get("linkedin_url"),
        github_url: row.get("github_url"),
        portfolio_url: row.get("portfolio_url"),
        created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc),
    })
}

/// SQLite implementation of ExperienceRepository
pub struct SqliteExperienceRepository {
    db: Database,
}

impl SqliteExperienceRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ExperienceRepository for SqliteExperienceRepository {
    async fn create(&self, experience: &Experience) -> Result<Experience> {
        let achievements_json = serde_json::to_string(&experience.achievements)?;
        
        sqlx::query(
            r#"
            INSERT INTO experiences (id, profile_id, company, title, location, start_date, 
                end_date, is_current, description, highlights, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(experience.id.to_string())
        .bind(experience.profile_id.to_string())
        .bind(&experience.company)
        .bind(&experience.position)
        .bind(&experience.location)
        .bind(experience.start_date.to_rfc3339())
        .bind(experience.end_date.map(|d| d.to_rfc3339()))
        .bind(experience.current)
        .bind(&experience.description)
        .bind(&achievements_json)
        .bind(experience.created_at.to_rfc3339())
        .bind(experience.updated_at.to_rfc3339())
        .execute(self.db.pool())
        .await?;
        
        Ok(experience.clone())
    }
    
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Experience>> {
        let row = sqlx::query("SELECT * FROM experiences WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(self.db.pool())
            .await?;
        
        match row {
            Some(row) => Ok(Some(row_to_experience(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn get_by_profile_id(&self, profile_id: &Uuid) -> Result<Vec<Experience>> {
        let rows = sqlx::query("SELECT * FROM experiences WHERE profile_id = ? ORDER BY start_date DESC")
            .bind(profile_id.to_string())
            .fetch_all(self.db.pool())
            .await?;
        
        let mut experiences = Vec::new();
        for row in rows {
            experiences.push(row_to_experience(&row)?);
        }
        Ok(experiences)
    }
    
    async fn update(&self, experience: &Experience) -> Result<Experience> {
        let achievements_json = serde_json::to_string(&experience.achievements)?;
        
        sqlx::query(
            r#"
            UPDATE experiences SET 
                company = ?, title = ?, location = ?, start_date = ?, end_date = ?,
                is_current = ?, description = ?, highlights = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&experience.company)
        .bind(&experience.position)
        .bind(&experience.location)
        .bind(experience.start_date.to_rfc3339())
        .bind(experience.end_date.map(|d| d.to_rfc3339()))
        .bind(experience.current)
        .bind(&experience.description)
        .bind(&achievements_json)
        .bind(Utc::now().to_rfc3339())
        .bind(experience.id.to_string())
        .execute(self.db.pool())
        .await?;
        
        Ok(experience.clone())
    }
    
    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM experiences WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
}

/// Helper to convert a database row to an Experience struct
fn row_to_experience(row: &sqlx::sqlite::SqliteRow) -> Result<Experience> {
    let id_str: String = row.get("id");
    let profile_id_str: String = row.get("profile_id");
    let start_date_str: String = row.get("start_date");
    let end_date_str: Option<String> = row.get("end_date");
    let created_at_str: String = row.get("created_at");
    let updated_at_str: String = row.get("updated_at");
    let highlights_json: Option<String> = row.get("highlights");
    
    let achievements: Vec<String> = highlights_json
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    
    Ok(Experience {
        id: Uuid::parse_str(&id_str)?,
        profile_id: Uuid::parse_str(&profile_id_str)?,
        company: row.get("company"),
        position: row.get("title"),
        location: row.get("location"),
        start_date: DateTime::parse_from_rfc3339(&start_date_str)?.with_timezone(&Utc),
        end_date: end_date_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
        current: row.get::<i32, _>("is_current") != 0,
        description: row.get::<Option<String>, _>("description").unwrap_or_default(),
        achievements,
        created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc),
    })
}

/// SQLite implementation of EducationRepository
pub struct SqliteEducationRepository {
    db: Database,
}

impl SqliteEducationRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EducationRepository for SqliteEducationRepository {
    async fn create(&self, education: &Education) -> Result<Education> {
        sqlx::query(
            r#"
            INSERT INTO education (id, profile_id, institution, degree, field_of_study, 
                start_date, end_date, gpa, description, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(education.id.to_string())
        .bind(education.profile_id.to_string())
        .bind(&education.institution)
        .bind(&education.degree)
        .bind(&education.field)
        .bind(education.start_date.to_rfc3339())
        .bind(education.end_date.map(|d| d.to_rfc3339()))
        .bind(education.gpa.map(|g| g.to_string()))
        .bind(serde_json::to_string(&education.honors).unwrap_or_default())
        .bind(education.created_at.to_rfc3339())
        .bind(education.updated_at.to_rfc3339())
        .execute(self.db.pool())
        .await?;
        
        Ok(education.clone())
    }
    
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Education>> {
        let row = sqlx::query("SELECT * FROM education WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(self.db.pool())
            .await?;
        
        match row {
            Some(row) => Ok(Some(row_to_education(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn get_by_profile_id(&self, profile_id: &Uuid) -> Result<Vec<Education>> {
        let rows = sqlx::query("SELECT * FROM education WHERE profile_id = ? ORDER BY start_date DESC")
            .bind(profile_id.to_string())
            .fetch_all(self.db.pool())
            .await?;
        
        let mut education = Vec::new();
        for row in rows {
            education.push(row_to_education(&row)?);
        }
        Ok(education)
    }
    
    async fn update(&self, education: &Education) -> Result<Education> {
        sqlx::query(
            r#"
            UPDATE education SET 
                institution = ?, degree = ?, field_of_study = ?, start_date = ?, end_date = ?,
                gpa = ?, description = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&education.institution)
        .bind(&education.degree)
        .bind(&education.field)
        .bind(education.start_date.to_rfc3339())
        .bind(education.end_date.map(|d| d.to_rfc3339()))
        .bind(education.gpa.map(|g| g.to_string()))
        .bind(serde_json::to_string(&education.honors).unwrap_or_default())
        .bind(Utc::now().to_rfc3339())
        .bind(education.id.to_string())
        .execute(self.db.pool())
        .await?;
        
        Ok(education.clone())
    }
    
    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM education WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
}

/// Helper to convert a database row to an Education struct
fn row_to_education(row: &sqlx::sqlite::SqliteRow) -> Result<Education> {
    let id_str: String = row.get("id");
    let profile_id_str: String = row.get("profile_id");
    let start_date_str: Option<String> = row.get("start_date");
    let end_date_str: Option<String> = row.get("end_date");
    let created_at_str: String = row.get("created_at");
    let updated_at_str: String = row.get("updated_at");
    let gpa_str: Option<String> = row.get("gpa");
    let description: Option<String> = row.get("description");
    
    let honors: Vec<String> = description
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    
    Ok(Education {
        id: Uuid::parse_str(&id_str)?,
        profile_id: Uuid::parse_str(&profile_id_str)?,
        institution: row.get("institution"),
        degree: row.get("degree"),
        field: row.get::<Option<String>, _>("field_of_study").unwrap_or_default(),
        location: None,
        start_date: start_date_str
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc)))
            .unwrap_or_else(Utc::now),
        end_date: end_date_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
        gpa: gpa_str.and_then(|s| s.parse().ok()),
        honors,
        created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc),
    })
}

/// SQLite implementation of SkillRepository
pub struct SqliteSkillRepository {
    db: Database,
}

impl SqliteSkillRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SkillRepository for SqliteSkillRepository {
    async fn create(&self, skill: &Skill) -> Result<Skill> {
        sqlx::query(
            r#"
            INSERT INTO skills (id, profile_id, name, category, level, years_experience, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(skill.id.to_string())
        .bind(skill.profile_id.to_string())
        .bind(&skill.name)
        .bind(format!("{:?}", skill.category))
        .bind(format!("{:?}", skill.proficiency))
        .bind(skill.years_experience.map(|y| y as i64))
        .bind(skill.created_at.to_rfc3339())
        .execute(self.db.pool())
        .await?;
        
        Ok(skill.clone())
    }
    
    async fn get_by_profile_id(&self, profile_id: &Uuid) -> Result<Vec<Skill>> {
        let rows = sqlx::query("SELECT * FROM skills WHERE profile_id = ? ORDER BY name")
            .bind(profile_id.to_string())
            .fetch_all(self.db.pool())
            .await?;
        
        let mut skills = Vec::new();
        for row in rows {
            skills.push(row_to_skill(&row)?);
        }
        Ok(skills)
    }
    
    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM skills WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
    
    async fn delete_by_profile_id(&self, profile_id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM skills WHERE profile_id = ?")
            .bind(profile_id.to_string())
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
}

/// Helper to convert a database row to a Skill struct
fn row_to_skill(row: &sqlx::sqlite::SqliteRow) -> Result<Skill> {
    let id_str: String = row.get("id");
    let profile_id_str: String = row.get("profile_id");
    let created_at_str: String = row.get("created_at");
    let category_str: Option<String> = row.get("category");
    let level_str: Option<String> = row.get("level");
    let years_exp: Option<i64> = row.get("years_experience");
    
    let category = match category_str.as_deref() {
        Some("Programming") => SkillCategory::Programming,
        Some("Framework") => SkillCategory::Framework,
        Some("Database") => SkillCategory::Database,
        Some("Tool") => SkillCategory::Tool,
        Some("Language") => SkillCategory::Language,
        Some("Soft") => SkillCategory::Soft,
        _ => SkillCategory::Other,
    };
    
    let proficiency = match level_str.as_deref() {
        Some("Beginner") => SkillLevel::Beginner,
        Some("Intermediate") => SkillLevel::Intermediate,
        Some("Advanced") => SkillLevel::Advanced,
        Some("Expert") => SkillLevel::Expert,
        _ => SkillLevel::Intermediate,
    };
    
    Ok(Skill {
        id: Uuid::parse_str(&id_str)?,
        profile_id: Uuid::parse_str(&profile_id_str)?,
        name: row.get("name"),
        category,
        proficiency,
        years_experience: years_exp.map(|y| y as u32),
        created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
    })
}

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
            JobSource::Remotive => "Remotive",
            JobSource::HNWhoIsHiring => "HN Who's Hiring",
            JobSource::Arbeitnow => "Arbeitnow",
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
                JobSource::Remotive => "Remotive".to_string(),
                JobSource::HNWhoIsHiring => "HN Who's Hiring".to_string(),
                JobSource::Arbeitnow => "Arbeitnow".to_string(),
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
        "Remotive" => JobSource::Remotive,
        "HN Who's Hiring" => JobSource::HNWhoIsHiring,
        "Arbeitnow" => JobSource::Arbeitnow,
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
