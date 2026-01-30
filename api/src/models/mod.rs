pub mod user;
pub mod profile;
pub mod experience;
pub mod education;
pub mod skill;
pub mod project;
pub mod job;
pub mod job_application;
pub mod resume;
pub mod resume_analysis;

// Re-export all models for easy importing
pub use user::User;
pub use profile::{Profile, FullProfile};
pub use experience::Experience;
pub use education::Education;
pub use skill::{Skill, SkillCategory, SkillLevel};
pub use project::Project;
pub use job::{Job, JobSource, SalaryRange, SalaryPeriod};
pub use job_application::{JobApplication, ApplicationStatus};
pub use resume::{Resume, ResumeTemplate, CustomSection};
pub use resume_analysis::{ResumeAnalysis, Suggestion, SuggestionCategory, Priority};