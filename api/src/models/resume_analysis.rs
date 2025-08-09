use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResumeAnalysis {
    pub id: Uuid,
    pub resume_id: Uuid,
    pub score: u32,
    pub suggestions: Vec<Suggestion>,
    pub keyword_match: f32,
    pub ats_compatibility: f32,
    pub analyzed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Suggestion {
    pub category: SuggestionCategory,
    pub priority: Priority,
    pub message: String,
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuggestionCategory {
    Content,
    Formatting,
    Keywords,
    Structure,
    Grammar,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}