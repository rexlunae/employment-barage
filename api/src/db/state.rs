//! Global application state for server-side

use super::Database;
use std::sync::OnceLock;
use tokio::sync::OnceCell;

static DATABASE: OnceCell<Database> = OnceCell::const_new();

/// Initialize the global database connection
pub async fn init_database(db_path: &str) -> anyhow::Result<()> {
    let db = Database::new(db_path).await?;
    DATABASE.set(db).map_err(|_| anyhow::anyhow!("Database already initialized"))?;
    Ok(())
}

/// Get a reference to the global database
/// 
/// Panics if the database has not been initialized
pub fn get_database() -> &'static Database {
    DATABASE.get().expect("Database not initialized. Call init_database() first.")
}

/// Check if the database has been initialized
pub fn is_database_initialized() -> bool {
    DATABASE.get().is_some()
}
