use dioxus::prelude::*;
use crate::models::*;
use anyhow::Result;

#[cfg(not(target_arch = "wasm32"))]
use crate::db::{get_database, SqliteJobRepository, JobRepository, JobSearchQuery};

/// Search for jobs across multiple platforms
#[server(SearchJobs)]
pub async fn search_jobs(
    keywords: String,
    location: String,
    salary_min: Option<u32>,
    sources: Vec<JobSource>
) -> Result<Vec<Job>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let db = get_database();
        let repo = SqliteJobRepository::new(db.clone());
        
        let query = JobSearchQuery {
            keywords: if keywords.is_empty() { None } else { Some(keywords) },
            location: if location.is_empty() { None } else { Some(location) },
            min_salary: salary_min,
            sources,
            remote_only: false,
            limit: Some(50),
            offset: None,
        };
        
        repo.search(&query)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        Ok(Vec::new())
    }
}

/// Get saved jobs
#[server(GetSavedJobs)]
pub async fn get_saved_jobs() -> Result<Vec<Job>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let db = get_database();
        let repo = SqliteJobRepository::new(db.clone());
        
        repo.get_saved()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        Ok(Vec::new())
    }
}

/// Save a job
#[server(SaveJob)]
pub async fn save_job(job_id: String) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let db = get_database();
        let repo = SqliteJobRepository::new(db.clone());
        let id = uuid::Uuid::parse_str(&job_id).map_err(|e| ServerFnError::new(e.to_string()))?;
        
        repo.save(&id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        Ok(())
    }
}

/// Unsave a job
#[server(UnsaveJob)]
pub async fn unsave_job(job_id: String) -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let db = get_database();
        let repo = SqliteJobRepository::new(db.clone());
        let id = uuid::Uuid::parse_str(&job_id).map_err(|e| ServerFnError::new(e.to_string()))?;
        
        repo.unsave(&id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        Ok(())
    }
}

/// Generate a personalized cover letter for a job
#[server(GenerateCoverLetter)]
pub async fn generate_cover_letter(
    job_id: String,
    user_profile_id: String,
    tone: CoverLetterTone
) -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::services::ai_service::{AiProvider, generate_cover_letter as ai_generate};
        
        // Use local Ollama by default
        let provider = AiProvider::default();
        
        // TODO: Fetch actual job and profile from database
        let job_title = "Software Engineer";
        let company = "Tech Company";
        let job_description = "Looking for an experienced developer...";
        let resume_summary = "Experienced software engineer with 15+ years in Python, Rust, and web development.";
        
        let tone_str = match tone {
            CoverLetterTone::Professional => "professional and formal",
            CoverLetterTone::Friendly => "friendly and conversational",
            CoverLetterTone::Enthusiastic => "enthusiastic and energetic",
        };
        
        ai_generate(&provider, job_title, company, job_description, resume_summary, tone_str)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // Fallback for WASM - use template
        let cover_letter = match tone {
            CoverLetterTone::Professional => generate_professional_cover_letter(&job_id, &user_profile_id).await,
            CoverLetterTone::Friendly => generate_friendly_cover_letter(&job_id, &user_profile_id).await,
            CoverLetterTone::Enthusiastic => generate_enthusiastic_cover_letter(&job_id, &user_profile_id).await,
        };
        cover_letter.map_err(|e| ServerFnError::new(e.to_string()))
    }
}

/// Apply to a job automatically
#[server(ApplyToJob)]
pub async fn apply_to_job(
    job_id: String,
    user_id: String,
    cover_letter: String,
    resume_id: String,
    auto_submit: bool
) -> Result<JobApplication, ServerFnError> {
    // TODO: Save to database and potentially trigger browser automation
    
    Ok(JobApplication {
        id: uuid::Uuid::new_v4(),
        user_id: uuid::Uuid::parse_str(&user_id).map_err(|e| ServerFnError::new(e.to_string()))?,
        job_id: uuid::Uuid::parse_str(&job_id).map_err(|e| ServerFnError::new(e.to_string()))?,
        status: if auto_submit { ApplicationStatus::Applied } else { ApplicationStatus::Draft },
        applied_date: chrono::Utc::now(),
        cover_letter: Some(cover_letter),
        custom_resume_id: Some(uuid::Uuid::parse_str(&resume_id).map_err(|e| ServerFnError::new(e.to_string()))?),
        notes: None,
        follow_up_date: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}

/// Get user's job applications
#[server(GetUserApplications)]
pub async fn get_user_applications(user_id: String) -> Result<Vec<JobApplication>, ServerFnError> {
    // TODO: Query from database
    Ok(Vec::new())
}

/// Update application status
#[server(UpdateApplicationStatus)]
pub async fn update_application_status(
    application_id: String,
    status: ApplicationStatus,
    notes: Option<String>
) -> Result<JobApplication, ServerFnError> {
    // TODO: Update in database
    Ok(JobApplication {
        id: uuid::Uuid::parse_str(&application_id).map_err(|e| ServerFnError::new(e.to_string()))?,
        user_id: uuid::Uuid::new_v4(),
        job_id: uuid::Uuid::new_v4(),
        status,
        applied_date: chrono::Utc::now(),
        cover_letter: None,
        custom_resume_id: None,
        notes,
        follow_up_date: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CoverLetterTone {
    Professional,
    Friendly,
    Enthusiastic,
}

async fn generate_professional_cover_letter(
    _job_id: &str,
    _profile_id: &str
) -> Result<String> {
    Ok(r#"
Dear Hiring Manager,

I am writing to express my strong interest in the [Position Title] role at [Company Name]. With my extensive background in software development and proven track record of delivering high-quality solutions, I am confident that I would be a valuable addition to your team.

In my previous role at [Previous Company], I successfully [specific achievement that relates to the job requirements]. This experience has equipped me with the technical skills and problem-solving abilities that directly align with the requirements outlined in your job posting.

I am particularly drawn to [Company Name] because of [specific reason related to the company/role]. I believe my expertise in [relevant skills/technologies] would contribute significantly to your team's continued success.

Thank you for considering my application. I look forward to the opportunity to discuss how my skills and experience can benefit your organization.

Sincerely,
[Your Name]
"#.to_string())
}

async fn generate_friendly_cover_letter(
    _job_id: &str,
    _profile_id: &str
) -> Result<String> {
    Ok(r#"
Hello [Hiring Manager Name / Team],

I hope this message finds you well! I'm excited to apply for the [Position Title] position at [Company Name]. After learning about your team's innovative work in [relevant area], I knew I had to reach out.

Throughout my career, I've had the pleasure of working on projects that mirror what you're doing at [Company Name]. For instance, at [Previous Company], I [relevant experience/achievement]. This experience taught me [relevant lesson/skill] and reinforced my passion for [relevant field/technology].

What really draws me to [Company Name] is [specific reason]. I love how you approach [specific aspect of their work/culture], and I'd be thrilled to contribute my skills in [relevant areas] to help the team achieve even greater things.

I'd love the chance to chat more about how I can contribute to your team's success. Thanks so much for your time and consideration!

Best regards,
[Your Name]
"#.to_string())
}

async fn generate_enthusiastic_cover_letter(
    _job_id: &str,
    _profile_id: &str
) -> Result<String> {
    Ok(r#"
Dear [Company Name] Team,

I am absolutely thrilled to apply for the [Position Title] role! The moment I saw this opportunity, I knew it was the perfect next step in my career journey.

Your mission to [company mission/values] resonates deeply with me, and I'm incredibly excited about the possibility of contributing to such meaningful work. My experience with [relevant technology/field] at [Previous Company] has prepared me well for this role, and I can't wait to bring that expertise to your innovative team.

What excites me most about this position is [specific aspect of the role]. I'm passionate about [relevant area], and the opportunity to [specific responsibility/project] would be a dream come true. I know that my enthusiasm, combined with my technical skills in [relevant skills], would make me a fantastic addition to your team.

I'm eager to learn more about how I can help [Company Name] achieve its goals and make a real impact in [relevant field/industry]. Thank you for considering my application – I'm looking forward to the possibility of joining your amazing team!

With enthusiasm and gratitude,
[Your Name]
"#.to_string())
}
