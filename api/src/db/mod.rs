//! Database abstraction layer
//! 
//! Uses SQLite for local storage with a repository pattern
//! that can be swapped to Postgres/Supabase later.

#[cfg(not(target_arch = "wasm32"))]
pub mod sqlite;

#[cfg(not(target_arch = "wasm32"))]
pub mod repository;

#[cfg(not(target_arch = "wasm32"))]
pub mod sqlite_impl;

#[cfg(not(target_arch = "wasm32"))]
pub mod state;

#[cfg(not(target_arch = "wasm32"))]
pub use repository::*;

#[cfg(not(target_arch = "wasm32"))]
pub use sqlite::Database;

#[cfg(not(target_arch = "wasm32"))]
pub use sqlite_impl::*;

#[cfg(not(target_arch = "wasm32"))]
pub use state::*;

#[cfg(not(target_arch = "wasm32"))]
pub use sqlite::default_db_path;
