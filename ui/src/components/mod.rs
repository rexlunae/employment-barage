pub mod dashboard;
pub mod upload;
pub mod hero;
pub mod navbar;
pub mod echo;

// Re-export components for easy importing
pub use dashboard::Dashboard;
pub use upload::ResumeUpload;
pub use hero::Hero;
pub use navbar::Navbar;
pub use echo::Echo;