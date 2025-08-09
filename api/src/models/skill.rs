use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub name: String,
    pub category: SkillCategory,
    pub proficiency: SkillLevel,
    pub years_experience: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillCategory {
    Programming,
    Framework,
    Database,
    Tool,
    Language,
    Soft,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}