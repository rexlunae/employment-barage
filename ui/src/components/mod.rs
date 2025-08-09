pub mod dashboard;
pub mod upload;
pub mod navbar;
pub mod app;

// Re-export components for easy importing
pub use dashboard::Dashboard;
pub use upload::ResumeUpload;
pub use navbar::Navbar;
pub use app::{App, SimpleApp};