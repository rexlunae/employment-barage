use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: Uuid,
    pub user_id: Uuid,
    pub job_id: Uuid,
    pub status: ApplicationStatus,
    pub applied_date: DateTime<Utc>,
    pub cover_letter: Option<String>,
    pub custom_resume_id: Option<Uuid>,
    pub notes: Option<String>,
    pub follow_up_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Draft,
    Applied,
    Interviewing,
    Offered,
    Rejected,
    Withdrawn,
}