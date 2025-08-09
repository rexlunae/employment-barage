use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{Profile, Experience, Education, Skill, SkillCategory, SkillLevel, Project};

pub struct ResumeParser {
    email_regex: Regex,
    phone_regex: Regex,
    date_regex: Regex,
    url_regex: Regex,
}

impl ResumeParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            email_regex: Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")?,
            phone_regex: Regex::new(r"(\+?1?[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})")?,
            date_regex: Regex::new(r"(?i)(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*[\s,]*(\d{4})|(\d{1,2})[\/\-](\d{4})|(\d{4})")?,
            url_regex: Regex::new(r"https?://[^\s]+")?,
        })
    }

    pub async fn parse_pdf(&self, _pdf_data: &[u8]) -> Result<ParsedResume> {
        // Temporarily disabled - would use pdf_extract::extract_text_from_mem
        let text = "Sample extracted text from PDF";
        self.parse_text(text).await
    }

    pub async fn parse_docx(&self, _docx_data: &[u8]) -> Result<ParsedResume> {
        // Temporarily disabled - would use docx_rs::read_docx
        let text = "Sample extracted text from DOCX";
        self.parse_text(text).await
    }

    // DOCX parsing functions temporarily removed to resolve getrandom conflicts

    async fn parse_text(&self, text: &str) -> Result<ParsedResume> {
        let sections = self.identify_sections(text);
        
        Ok(ParsedResume {
            profile: self.extract_profile(text, &sections)?,
            experiences: self.extract_experiences(text, &sections)?,
            education: self.extract_education(text, &sections)?,
            skills: self.extract_skills(text, &sections)?,
            projects: self.extract_projects(text, &sections)?,
        })
    }

    fn identify_sections(&self, text: &str) -> HashMap<String, (usize, usize)> {
        let mut sections = HashMap::new();
        let lines: Vec<&str> = text.lines().collect();
        
        let section_headers = [
            ("contact", vec!["contact", "personal information", "details"]),
            ("summary", vec!["summary", "objective", "profile", "about"]),
            ("experience", vec!["experience", "work experience", "employment", "professional experience", "work history"]),
            ("education", vec!["education", "academic background", "qualifications"]),
            ("skills", vec!["skills", "technical skills", "competencies", "proficiencies"]),
            ("projects", vec!["projects", "personal projects", "portfolio"]),
        ];

        for (section_name, headers) in section_headers {
            for (i, line) in lines.iter().enumerate() {
                let line_lower = line.to_lowercase();
                if headers.iter().any(|header| line_lower.contains(header) && line.trim().len() < 50) {
                    let start = text.lines().take(i).map(|l| l.len() + 1).sum::<usize>();
                    let end = text.len(); // Will be adjusted when next section is found
                    sections.insert(section_name.to_string(), (start, end));
                    break;
                }
            }
        }

        sections
    }

    fn extract_profile(&self, text: &str, _sections: &HashMap<String, (usize, usize)>) -> Result<Profile> {
        let email = self.email_regex.find(text)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow!("No email found in resume"))?;

        let phone = self.phone_regex.find(text)
            .map(|m| m.as_str().to_string());

        let linkedin_url = self.extract_url(text, "linkedin.com");
        let github_url = self.extract_url(text, "github.com");
        let portfolio_url = self.extract_url(text, "portfolio")
            .or_else(|| self.extract_url(text, "website"));

        let _name = self.extract_name(text)?;
        let location = self.extract_location(text);

        Ok(Profile {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(), // Will be set by the caller
            summary: None, // Will be extracted from summary section
            phone,
            email,
            location,
            linkedin_url,
            github_url,
            portfolio_url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    fn extract_name(&self, text: &str) -> Result<String> {
        // Simple heuristic: assume name is in the first few lines
        let lines: Vec<&str> = text.lines().take(5).collect();
        
        for line in lines {
            let line = line.trim();
            if line.len() > 2 && line.len() < 50 && 
               !line.contains('@') && !line.contains("http") &&
               !self.phone_regex.is_match(line) {
                // Check if it looks like a name (contains spaces or is a single word with caps)
                if line.contains(' ') || (line.chars().next().unwrap_or('a').is_uppercase() && line.len() > 3) {
                    return Ok(line.to_string());
                }
            }
        }
        
        Err(anyhow!("Could not extract name from resume"))
    }

    fn extract_location(&self, text: &str) -> Option<String> {
        // Simple location extraction - look for patterns like "City, State" or "City, Country"
        let location_regex = Regex::new(r"([A-Za-z\s]+),\s*([A-Za-z\s]+)").ok()?;
        location_regex.find(text).map(|m| m.as_str().to_string())
    }

    fn extract_url(&self, text: &str, domain_hint: &str) -> Option<String> {
        for url_match in self.url_regex.find_iter(text) {
            let url = url_match.as_str();
            if url.contains(domain_hint) {
                return Some(url.to_string());
            }
        }
        None
    }

    fn extract_experiences(&self, text: &str, sections: &HashMap<String, (usize, usize)>) -> Result<Vec<Experience>> {
        let mut experiences = Vec::new();
        
        if let Some((start, end)) = sections.get("experience") {
            let experience_text = &text[*start..*end];
            // Simple parsing - in reality, this would be much more sophisticated
            let lines: Vec<&str> = experience_text.lines().collect();
            
            let mut current_experience: Option<ExperienceBuilder> = None;
            
            for line in lines {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                // Check if this looks like a job title line (contains company or position indicators)
                if self.looks_like_job_header(line) {
                    if let Some(exp) = current_experience.take() {
                        if let Ok(experience) = exp.build() {
                            experiences.push(experience);
                        }
                    }
                    current_experience = Some(self.parse_job_header(line));
                } else if let Some(ref mut exp) = current_experience {
                    // Add to description
                    exp.add_description_line(line);
                }
            }
            
            // Don't forget the last experience
            if let Some(exp) = current_experience {
                if let Ok(experience) = exp.build() {
                    experiences.push(experience);
                }
            }
        }
        
        Ok(experiences)
    }

    fn looks_like_job_header(&self, line: &str) -> bool {
        // Heuristic: job headers often contain company names, dates, or position titles
        self.date_regex.is_match(line) || 
        line.contains("Inc") || line.contains("LLC") || line.contains("Corp") ||
        line.split_whitespace().count() <= 6 // Short lines are often headers
    }

    fn parse_job_header(&self, line: &str) -> ExperienceBuilder {
        // Very basic parsing - would need much more sophistication in practice
        let parts: Vec<&str> = line.split(", ").collect();
        let company = parts.first().unwrap_or(&"Unknown Company").to_string();
        let position = parts.get(1).unwrap_or(&"Unknown Position").to_string();
        
        ExperienceBuilder {
            id: Uuid::new_v4(),
            profile_id: Uuid::new_v4(),
            company,
            position,
            location: None,
            start_date: Utc::now(), // Would parse from text
            end_date: None,
            current: false,
            description: String::new(),
            achievements: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn extract_education(&self, _text: &str, _sections: &HashMap<String, (usize, usize)>) -> Result<Vec<Education>> {
        // Simplified for now
        Ok(Vec::new())
    }

    fn extract_skills(&self, text: &str, sections: &HashMap<String, (usize, usize)>) -> Result<Vec<Skill>> {
        let mut skills = Vec::new();
        
        if let Some((start, end)) = sections.get("skills") {
            let skills_text = &text[*start..*end];
            let words: Vec<&str> = skills_text.split_whitespace().collect();
            
            // Simple skill extraction - look for technology names
            let common_skills = [
                "Python", "JavaScript", "Java", "C++", "C#", "Go", "Rust", "TypeScript",
                "React", "Angular", "Vue", "Django", "Flask", "Express", "Spring",
                "PostgreSQL", "MySQL", "MongoDB", "Redis", "Docker", "Kubernetes",
                "AWS", "Azure", "GCP", "Git", "Linux", "HTML", "CSS"
            ];
            
            for word in words {
                let word = word.trim_matches(|c: char| !c.is_alphanumeric());
                if common_skills.contains(&word) {
                    skills.push(Skill {
                        id: Uuid::new_v4(),
                        profile_id: Uuid::new_v4(),
                        name: word.to_string(),
                        category: self.categorize_skill(word),
                        proficiency: SkillLevel::Intermediate, // Default
                        years_experience: None,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    });
                }
            }
        }
        
        Ok(skills)
    }

    fn categorize_skill(&self, skill: &str) -> SkillCategory {
        match skill {
            "Python" | "JavaScript" | "Java" | "C++" | "C#" | "Go" | "Rust" | "TypeScript" => SkillCategory::Programming,
            "React" | "Angular" | "Vue" | "Django" | "Flask" | "Express" | "Spring" => SkillCategory::Framework,
            "PostgreSQL" | "MySQL" | "MongoDB" | "Redis" => SkillCategory::Database,
            "Docker" | "Kubernetes" | "Git" | "Linux" => SkillCategory::Tool,
            _ => SkillCategory::Other,
        }
    }

    fn extract_projects(&self, _text: &str, _sections: &HashMap<String, (usize, usize)>) -> Result<Vec<Project>> {
        // Simplified for now
        Ok(Vec::new())
    }
}

struct ExperienceBuilder {
    id: Uuid,
    profile_id: Uuid,
    company: String,
    position: String,
    location: Option<String>,
    start_date: DateTime<Utc>,
    end_date: Option<DateTime<Utc>>,
    current: bool,
    description: String,
    achievements: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ExperienceBuilder {
    fn add_description_line(&mut self, line: &str) {
        if !self.description.is_empty() {
            self.description.push('\n');
        }
        self.description.push_str(line);
    }
    
    fn build(self) -> Result<Experience> {
        Ok(Experience {
            id: self.id,
            profile_id: self.profile_id,
            company: self.company,
            position: self.position,
            location: self.location,
            start_date: self.start_date,
            end_date: self.end_date,
            current: self.current,
            description: self.description,
            achievements: self.achievements,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedResume {
    pub profile: Profile,
    pub experiences: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub projects: Vec<Project>,
}