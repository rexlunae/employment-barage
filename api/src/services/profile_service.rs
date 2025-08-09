use dioxus::prelude::*;
use crate::models::*;
use crate::resume_parser::ParsedResume;

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

/// Get user profile
#[server(GetProfile)]
pub async fn get_profile(user_id: String) -> Result<Option<Profile>, ServerFnError> {
    // In a real implementation, this would query the database
    // For now, return a dummy profile
    Ok(None)
}

/// Save user profile
#[server(SaveProfile)]
pub async fn save_profile(profile: Profile) -> Result<Profile, ServerFnError> {
    // In a real implementation, this would save to database
    Ok(profile)
}

/// Get user experiences
#[server(GetExperiences)]
pub async fn get_experiences(profile_id: String) -> Result<Vec<Experience>, ServerFnError> {
    // In a real implementation, this would query the database
    Ok(Vec::new())
}

/// Save experience
#[server(SaveExperience)]
pub async fn save_experience(experience: Experience) -> Result<Experience, ServerFnError> {
    // In a real implementation, this would save to database
    Ok(experience)
}

/// Get user education
#[server(GetEducation)]
pub async fn get_education(profile_id: String) -> Result<Vec<Education>, ServerFnError> {
    Ok(Vec::new())
}

/// Save education
#[server(SaveEducation)]
pub async fn save_education(education: Education) -> Result<Education, ServerFnError> {
    Ok(education)
}

/// Get user skills
#[server(GetSkills)]
pub async fn get_skills(profile_id: String) -> Result<Vec<Skill>, ServerFnError> {
    Ok(Vec::new())
}

/// Save skill
#[server(SaveSkill)]
pub async fn save_skill(skill: Skill) -> Result<Skill, ServerFnError> {
    Ok(skill)
}

/// Get user projects
#[server(GetProjects)]
pub async fn get_projects(profile_id: String) -> Result<Vec<Project>, ServerFnError> {
    Ok(Vec::new())
}

/// Save project
#[server(SaveProject)]
pub async fn save_project(project: Project) -> Result<Project, ServerFnError> {
    Ok(project)
}