//! Repository traits and implementations
//! 
//! These traits define the interface for data access, allowing
//! easy swapping between SQLite, Postgres, or other backends.

use crate::models::*;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

/// Repository for user operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<User>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

/// Repository for profile operations
#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(&self, profile: &Profile) -> Result<Profile>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Profile>>;
    async fn get_by_user_id(&self, user_id: &Uuid) -> Result<Option<Profile>>;
    async fn update(&self, profile: &Profile) -> Result<Profile>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

/// Repository for experience operations
#[async_trait]
pub trait ExperienceRepository: Send + Sync {
    async fn create(&self, experience: &Experience) -> Result<Experience>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Experience>>;
    async fn get_by_profile_id(&self, profile_id: &Uuid) -> Result<Vec<Experience>>;
    async fn update(&self, experience: &Experience) -> Result<Experience>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

/// Repository for education operations
#[async_trait]
pub trait EducationRepository: Send + Sync {
    async fn create(&self, education: &Education) -> Result<Education>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Education>>;
    async fn get_by_profile_id(&self, profile_id: &Uuid) -> Result<Vec<Education>>;
    async fn update(&self, education: &Education) -> Result<Education>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

/// Repository for skills operations
#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn create(&self, skill: &Skill) -> Result<Skill>;
    async fn get_by_profile_id(&self, profile_id: &Uuid) -> Result<Vec<Skill>>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
    async fn delete_by_profile_id(&self, profile_id: &Uuid) -> Result<()>;
}

/// Repository for job operations
#[async_trait]
pub trait JobRepository: Send + Sync {
    async fn create(&self, job: &Job) -> Result<Job>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Job>>;
    async fn search(&self, query: &JobSearchQuery) -> Result<Vec<Job>>;
    async fn get_saved(&self) -> Result<Vec<Job>>;
    async fn save(&self, id: &Uuid) -> Result<()>;
    async fn unsave(&self, id: &Uuid) -> Result<()>;
    async fn update_match_score(&self, id: &Uuid, score: f64) -> Result<()>;
    async fn upsert_by_source(&self, job: &Job) -> Result<Job>;
}

/// Job search query parameters
#[derive(Debug, Clone, Default)]
pub struct JobSearchQuery {
    pub keywords: Option<String>,
    pub location: Option<String>,
    pub min_salary: Option<u32>,
    pub sources: Vec<JobSource>,
    pub remote_only: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Repository for job application operations
#[async_trait]
pub trait ApplicationRepository: Send + Sync {
    async fn create(&self, application: &JobApplication) -> Result<JobApplication>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<JobApplication>>;
    async fn get_by_user_id(&self, user_id: &Uuid) -> Result<Vec<JobApplication>>;
    async fn get_by_user_and_job(&self, user_id: &Uuid, job_id: &Uuid) -> Result<Option<JobApplication>>;
    async fn update(&self, application: &JobApplication) -> Result<JobApplication>;
    async fn update_status(&self, id: &Uuid, status: ApplicationStatus, notes: Option<String>) -> Result<JobApplication>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

/// Repository for resume operations
#[async_trait]
pub trait ResumeRepository: Send + Sync {
    async fn create(&self, resume: &Resume) -> Result<Resume>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Resume>>;
    async fn get_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Resume>>;
    async fn update(&self, resume: &Resume) -> Result<Resume>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}
