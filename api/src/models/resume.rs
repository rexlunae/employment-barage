use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub template: ResumeTemplate,
    pub profile_snapshot: Profile,
    pub selected_experiences: Vec<Uuid>,
    pub selected_projects: Vec<Uuid>,
    pub selected_skills: Vec<Uuid>,
    pub custom_sections: Vec<CustomSection>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResumeTemplate {
    Professional,
    Modern,
    Creative,
    Simple,
    Academic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSection {
    pub title: String,
    pub content: String,
    pub order: i32,
}