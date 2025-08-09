//! This crate contains all shared fullstack server functions and data models.
use dioxus::prelude::*;

pub mod models;
pub mod services;

// Re-export all models and services for easy access
pub use models::*;
pub use services::*;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
