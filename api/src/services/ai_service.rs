//! AI service for generating cover letters and resume suggestions
//! 
//! Supports multiple backends:
//! - Ollama (local, free)
//! - OpenAI (API)
//! - Claude (API)

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// AI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiProvider {
    Ollama { model: String, base_url: String },
    OpenAI { api_key: String, model: String },
    Claude { api_key: String, model: String },
}

impl Default for AiProvider {
    fn default() -> Self {
        AiProvider::Ollama {
            model: "phi3".to_string(),
            base_url: "http://localhost:11434".to_string(),
        }
    }
}

/// Request to Ollama API
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

/// Response from Ollama API
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

/// Generate text using the configured AI provider
pub async fn generate_text(provider: &AiProvider, prompt: &str) -> Result<String> {
    match provider {
        AiProvider::Ollama { model, base_url } => {
            generate_with_ollama(base_url, model, prompt).await
        }
        AiProvider::OpenAI { api_key, model } => {
            generate_with_openai(api_key, model, prompt).await
        }
        AiProvider::Claude { api_key, model } => {
            generate_with_claude(api_key, model, prompt).await
        }
    }
}

async fn generate_with_ollama(base_url: &str, model: &str, prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", base_url);
    
    let request = OllamaRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };
    
    let response = client
        .post(&url)
        .json(&request)
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .await?;
    
    let ollama_response: OllamaResponse = response.json().await?;
    Ok(ollama_response.response)
}

async fn generate_with_openai(api_key: &str, model: &str, prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    
    let request = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "user", "content": prompt}
        ],
        "max_tokens": 1000
    });
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    let json: serde_json::Value = response.json().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    Ok(content)
}

async fn generate_with_claude(api_key: &str, model: &str, prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    
    let request = serde_json::json!({
        "model": model,
        "max_tokens": 1000,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });
    
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    let json: serde_json::Value = response.json().await?;
    let content = json["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    Ok(content)
}

/// Generate a personalized cover letter
pub async fn generate_cover_letter(
    provider: &AiProvider,
    job_title: &str,
    company: &str,
    job_description: &str,
    resume_summary: &str,
    tone: &str,
) -> Result<String> {
    let prompt = format!(
        r#"Write a professional cover letter for the following job application. 

Job Title: {job_title}
Company: {company}
Job Description: {job_description}

Candidate Background:
{resume_summary}

Tone: {tone}

Requirements:
- Keep it under 400 words
- Highlight relevant experience that matches the job requirements
- Show enthusiasm for the company and role
- Include a strong opening and closing
- Make it personal and genuine, not generic
- Do not include placeholder text like [Your Name] - write as if from the candidate's perspective

Write only the cover letter text, starting with "Dear Hiring Manager," or similar greeting."#
    );
    
    generate_text(provider, &prompt).await
}

/// Analyze job-resume match and provide a score
pub async fn analyze_job_match(
    provider: &AiProvider,
    job_description: &str,
    resume_summary: &str,
) -> Result<JobMatchAnalysis> {
    let prompt = format!(
        r#"Analyze how well this candidate matches the job requirements.

Job Description:
{job_description}

Candidate Resume Summary:
{resume_summary}

Provide your analysis in the following JSON format:
{{
    "match_score": <0-100>,
    "strengths": ["strength1", "strength2", ...],
    "gaps": ["gap1", "gap2", ...],
    "suggestions": ["suggestion1", "suggestion2", ...]
}}

Be specific about which skills/experiences match and which are missing."#
    );
    
    let response = generate_text(provider, &prompt).await?;
    
    // Try to parse JSON from the response
    // The model might include extra text, so we try to find the JSON object
    let json_start = response.find('{').unwrap_or(0);
    let json_end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
    let json_str = &response[json_start..json_end];
    
    match serde_json::from_str::<JobMatchAnalysis>(json_str) {
        Ok(analysis) => Ok(analysis),
        Err(_) => {
            // If parsing fails, return a default analysis
            Ok(JobMatchAnalysis {
                match_score: 50,
                strengths: vec!["Analysis unavailable".to_string()],
                gaps: vec![],
                suggestions: vec!["Review job requirements manually".to_string()],
            })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMatchAnalysis {
    pub match_score: u32,
    pub strengths: Vec<String>,
    pub gaps: Vec<String>,
    pub suggestions: Vec<String>,
}
