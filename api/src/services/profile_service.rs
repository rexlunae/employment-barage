//! Profile management server functions
//! 
//! These functions handle profile CRUD operations and integrate with
//! the SQLite database for persistence.

use dioxus::prelude::*;
use crate::models::*;
use crate::resume_parser::ParsedResume;
use uuid::Uuid;
use chrono::Utc;

/// Upload and parse a resume file
#[server(UploadResume)]
pub async fn upload_resume(file_data: Vec<u8>, file_name: String, _user_id: String) -> Result<ParsedResume, ServerFnError> {
    use crate::resume_parser::ResumeParser;
    
    let parser = ResumeParser::new().map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let parsed = if file_name.ends_with(".pdf") {
        parser.parse_pdf(&file_data).await
    } else if file_name.ends_with(".docx") {
        parser.parse_docx(&file_data).await
    } else {
        return Err(ServerFnError::new("Unsupported file format".to_string()));
    };
    
    parsed.map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get user profile by user ID
#[server(GetProfile)]
pub async fn get_profile(user_id: String) -> Result<Option<Profile>, ServerFnError> {
    use crate::db::{get_database, SqliteProfileRepository, ProfileRepository};
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| ServerFnError::new(format!("Invalid user ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteProfileRepository::new(db.clone());
    
    repo.get_by_user_id(&user_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get profile by ID
#[server(GetProfileById)]
pub async fn get_profile_by_id(profile_id: String) -> Result<Option<Profile>, ServerFnError> {
    use crate::db::{get_database, SqliteProfileRepository, ProfileRepository};
    
    let profile_uuid = Uuid::parse_str(&profile_id)
        .map_err(|e| ServerFnError::new(format!("Invalid profile ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteProfileRepository::new(db.clone());
    
    repo.get_by_id(&profile_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Save or update user profile
#[server(SaveProfile)]
pub async fn save_profile(profile: Profile) -> Result<Profile, ServerFnError> {
    use crate::db::{get_database, SqliteProfileRepository, ProfileRepository};
    
    let db = get_database();
    let repo = SqliteProfileRepository::new(db.clone());
    
    // Check if profile exists
    let existing = repo.get_by_id(&profile.id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    if existing.is_some() {
        repo.update(&profile)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    } else {
        repo.create(&profile)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
}

/// Delete user profile
#[server(DeleteProfile)]
pub async fn delete_profile(profile_id: String) -> Result<(), ServerFnError> {
    use crate::db::{get_database, SqliteProfileRepository, ProfileRepository};
    
    let profile_uuid = Uuid::parse_str(&profile_id)
        .map_err(|e| ServerFnError::new(format!("Invalid profile ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteProfileRepository::new(db.clone());
    
    repo.delete(&profile_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get user experiences
#[server(GetExperiences)]
pub async fn get_experiences(profile_id: String) -> Result<Vec<Experience>, ServerFnError> {
    use crate::db::{get_database, SqliteExperienceRepository, ExperienceRepository};
    
    let profile_uuid = Uuid::parse_str(&profile_id)
        .map_err(|e| ServerFnError::new(format!("Invalid profile ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteExperienceRepository::new(db.clone());
    
    repo.get_by_profile_id(&profile_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Save or update experience
#[server(SaveExperience)]
pub async fn save_experience(experience: Experience) -> Result<Experience, ServerFnError> {
    use crate::db::{get_database, SqliteExperienceRepository, ExperienceRepository};
    
    let db = get_database();
    let repo = SqliteExperienceRepository::new(db.clone());
    
    // Check if experience exists
    let existing = repo.get_by_id(&experience.id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    if existing.is_some() {
        repo.update(&experience)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    } else {
        repo.create(&experience)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
}

/// Delete experience
#[server(DeleteExperience)]
pub async fn delete_experience(experience_id: String) -> Result<(), ServerFnError> {
    use crate::db::{get_database, SqliteExperienceRepository, ExperienceRepository};
    
    let experience_uuid = Uuid::parse_str(&experience_id)
        .map_err(|e| ServerFnError::new(format!("Invalid experience ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteExperienceRepository::new(db.clone());
    
    repo.delete(&experience_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get user education
#[server(GetEducation)]
pub async fn get_education(profile_id: String) -> Result<Vec<Education>, ServerFnError> {
    use crate::db::{get_database, SqliteEducationRepository, EducationRepository};
    
    let profile_uuid = Uuid::parse_str(&profile_id)
        .map_err(|e| ServerFnError::new(format!("Invalid profile ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteEducationRepository::new(db.clone());
    
    repo.get_by_profile_id(&profile_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Save or update education
#[server(SaveEducation)]
pub async fn save_education(education: Education) -> Result<Education, ServerFnError> {
    use crate::db::{get_database, SqliteEducationRepository, EducationRepository};
    
    let db = get_database();
    let repo = SqliteEducationRepository::new(db.clone());
    
    // Check if education exists
    let existing = repo.get_by_id(&education.id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    if existing.is_some() {
        repo.update(&education)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    } else {
        repo.create(&education)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
}

/// Delete education
#[server(DeleteEducation)]
pub async fn delete_education(education_id: String) -> Result<(), ServerFnError> {
    use crate::db::{get_database, SqliteEducationRepository, EducationRepository};
    
    let education_uuid = Uuid::parse_str(&education_id)
        .map_err(|e| ServerFnError::new(format!("Invalid education ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteEducationRepository::new(db.clone());
    
    repo.delete(&education_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get user skills
#[server(GetSkills)]
pub async fn get_skills(profile_id: String) -> Result<Vec<Skill>, ServerFnError> {
    use crate::db::{get_database, SqliteSkillRepository, SkillRepository};
    
    let profile_uuid = Uuid::parse_str(&profile_id)
        .map_err(|e| ServerFnError::new(format!("Invalid profile ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteSkillRepository::new(db.clone());
    
    repo.get_by_profile_id(&profile_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Save skill
#[server(SaveSkill)]
pub async fn save_skill(skill: Skill) -> Result<Skill, ServerFnError> {
    use crate::db::{get_database, SqliteSkillRepository, SkillRepository};
    
    let db = get_database();
    let repo = SqliteSkillRepository::new(db.clone());
    
    repo.create(&skill)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Delete skill
#[server(DeleteSkill)]
pub async fn delete_skill(skill_id: String) -> Result<(), ServerFnError> {
    use crate::db::{get_database, SqliteSkillRepository, SkillRepository};
    
    let skill_uuid = Uuid::parse_str(&skill_id)
        .map_err(|e| ServerFnError::new(format!("Invalid skill ID: {}", e)))?;
    
    let db = get_database();
    let repo = SqliteSkillRepository::new(db.clone());
    
    repo.delete(&skill_uuid)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get user projects (placeholder for now)
#[server(GetProjects)]
pub async fn get_projects(_profile_id: String) -> Result<Vec<Project>, ServerFnError> {
    // TODO: Implement project repository
    Ok(Vec::new())
}

/// Save project (placeholder for now)
#[server(SaveProject)]
pub async fn save_project(project: Project) -> Result<Project, ServerFnError> {
    // TODO: Implement project repository
    Ok(project)
}

/// Get full profile with all related data (for AI cover letter generation)
#[server(GetFullProfile)]
pub async fn get_full_profile(user_id: String) -> Result<Option<FullProfile>, ServerFnError> {
    use crate::db::{
        get_database, 
        SqliteProfileRepository, ProfileRepository,
        SqliteExperienceRepository, ExperienceRepository,
        SqliteEducationRepository, EducationRepository,
        SqliteSkillRepository, SkillRepository,
    };
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| ServerFnError::new(format!("Invalid user ID: {}", e)))?;
    
    let db = get_database();
    
    // Get profile
    let profile_repo = SqliteProfileRepository::new(db.clone());
    let profile = match profile_repo.get_by_user_id(&user_uuid).await
        .map_err(|e| ServerFnError::new(e.to_string()))? {
        Some(p) => p,
        None => return Ok(None),
    };
    
    // Get related data
    let exp_repo = SqliteExperienceRepository::new(db.clone());
    let experiences = exp_repo.get_by_profile_id(&profile.id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let edu_repo = SqliteEducationRepository::new(db.clone());
    let education = edu_repo.get_by_profile_id(&profile.id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let skill_repo = SqliteSkillRepository::new(db.clone());
    let skills = skill_repo.get_by_profile_id(&profile.id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(Some(FullProfile {
        profile,
        skills,
        experiences,
        education,
    }))
}

/// Create a new empty profile for a user
#[server(CreateEmptyProfile)]
pub async fn create_empty_profile(user_id: String, name: String, email: String) -> Result<Profile, ServerFnError> {
    use crate::db::{get_database, SqliteProfileRepository, ProfileRepository};
    
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| ServerFnError::new(format!("Invalid user ID: {}", e)))?;
    
    let profile = Profile {
        id: Uuid::new_v4(),
        user_id: user_uuid,
        name,
        email,
        headline: None,
        summary: None,
        phone: None,
        location: None,
        linkedin_url: None,
        github_url: None,
        portfolio_url: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let db = get_database();
    let repo = SqliteProfileRepository::new(db.clone());
    
    repo.create(&profile)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
