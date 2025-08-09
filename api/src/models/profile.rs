use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub summary: Option<String>,
    pub phone: Option<String>,
    pub email: String,
    pub location: Option<String>,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}