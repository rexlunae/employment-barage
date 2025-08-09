//! This crate contains all shared UI for the workspace.

pub mod components;

// Re-export main components for backward compatibility
pub use components::Dashboard;
pub use components::ResumeUpload;
pub use components::Hero;
pub use components::Navbar;
pub use components::Echo;
