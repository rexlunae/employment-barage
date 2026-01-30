//! SQLite database connection and initialization

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;
use anyhow::Result;

/// Database wrapper for SQLite connection pool
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection
    /// 
    /// If the database file doesn't exist, it will be created
    /// and migrations will be run automatically.
    pub async fn new(db_path: &str) -> Result<Self> {
        // Create parent directories if needed
        if let Some(parent) = Path::new(db_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let db_url = format!("sqlite:{}?mode=rwc", db_path);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        
        let db = Self { pool };
        db.run_migrations().await?;
        
        Ok(db)
    }
    
    /// Run database migrations
    async fn run_migrations(&self) -> Result<()> {
        sqlx::query(include_str!("migrations.sql"))
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    /// Get a reference to the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

/// Get the default database path
pub fn default_db_path() -> String {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("employment-barage");
    
    data_dir.join("data.db").to_string_lossy().to_string()
}
