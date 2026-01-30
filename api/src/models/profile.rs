use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub headline: Option<String>,
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

/// Complete profile data including related entities for AI cover letter generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FullProfile {
    pub profile: Profile,
    pub skills: Vec<crate::models::Skill>,
    pub experiences: Vec<crate::models::Experience>,
    pub education: Vec<crate::models::Education>,
}

impl FullProfile {
    /// Generate a summary suitable for AI cover letter generation
    pub fn to_resume_summary(&self) -> String {
        let mut summary = String::new();
        
        // Name and headline
        summary.push_str(&format!("Name: {}\n", self.profile.name));
        if let Some(headline) = &self.profile.headline {
            summary.push_str(&format!("Headline: {}\n", headline));
        }
        if let Some(loc) = &self.profile.location {
            summary.push_str(&format!("Location: {}\n", loc));
        }
        
        // Professional summary
        if let Some(prof_summary) = &self.profile.summary {
            summary.push_str(&format!("\nProfessional Summary:\n{}\n", prof_summary));
        }
        
        // Skills
        if !self.skills.is_empty() {
            summary.push_str("\nSkills:\n");
            for skill in &self.skills {
                summary.push_str(&format!("- {} ({:?})\n", skill.name, skill.proficiency));
            }
        }
        
        // Experience
        if !self.experiences.is_empty() {
            summary.push_str("\nWork Experience:\n");
            for exp in &self.experiences {
                summary.push_str(&format!(
                    "- {} at {} ({})\n  {}\n",
                    exp.position,
                    exp.company,
                    if exp.current { "Current".to_string() } else {
                        exp.end_date.map(|d| d.format("%Y").to_string()).unwrap_or_default()
                    },
                    exp.description
                ));
            }
        }
        
        // Education
        if !self.education.is_empty() {
            summary.push_str("\nEducation:\n");
            for edu in &self.education {
                summary.push_str(&format!(
                    "- {} in {} from {}\n",
                    edu.degree,
                    edu.field,
                    edu.institution
                ));
            }
        }
        
        summary
    }
}