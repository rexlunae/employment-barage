pub mod job_service;
pub mod profile_service;
pub mod resume_service;
pub mod resume_parser;

#[cfg(not(target_arch = "wasm32"))]
pub mod ai_service;

#[cfg(not(target_arch = "wasm32"))]
pub mod job_sources;

// Re-export server functions and types for easy importing
pub use job_service::{
    search_jobs, get_saved_jobs, save_job, unsave_job,
    generate_cover_letter, apply_to_job, get_user_applications, 
    update_application_status, CoverLetterTone,
    fetch_external_jobs, FetchJobsResult
};
pub use profile_service::*;
pub use resume_service::*;
pub use resume_parser::*;

// Job sources are server-side only
#[cfg(not(target_arch = "wasm32"))]
pub use job_sources::{JobSourceProvider, JobAggregator};

// AI service is available but not re-exported at top level to avoid conflicts
// Use api::services::ai_service::* when needed