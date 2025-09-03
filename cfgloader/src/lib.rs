//! # CFGLoader
//! 
//! A simple, powerful, and ergonomic configuration loading library for Rust applications.
//! 
//! CFGLoader automatically loads configuration from environment variables and `.env` files 
//! with compile-time validation and type safety.
//! 
//! ## Installation
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! cfgloader_rs = "0.1"
//! ```
//! 
//! ## Quick Start
//! 
//! ### Basic Usage
//! 
//! ```rust,no_run
//! use cfgloader_rs::*;
//! 
//! #[derive(FromEnv, Debug)]
//! struct Config {
//!     #[env("DATABASE_URL", default = "sqlite://app.db")]
//!     database_url: String,
//!     
//!     #[env("PORT", default = "8080")]
//!     port: u16,
//!     
//!     #[env("API_KEY", required)]
//!     api_key: String,
//!     
//!     #[env("FEATURES", default = "auth,logging", split = ",")]
//!     features: Vec<String>,
//! }
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::load(std::path::Path::new(".env"))?;
//!     println!("Config: {:#?}", config);
//!     Ok(())
//! }
//! ```
//! 
//! ### Nested Configuration
//! 
//! Organize your configuration into logical groups:
//! 
//! ```rust,no_run
//! use cfgloader_rs::*;
//! 
//! #[derive(FromEnv, Debug)]
//! struct AppConfig {
//!     #[env("APP_NAME", required)]
//!     name: String,
//!     
//!     // Nested structs automatically call their own load() method
//!     server: ServerConfig,
//!     database: DatabaseConfig,
//! }
//! 
//! #[derive(FromEnv, Debug)]
//! struct ServerConfig {
//!     #[env("SERVER_HOST", default = "127.0.0.1")]
//!     host: String,
//!     
//!     #[env("SERVER_PORT", default = "8080")]
//!     port: u16,
//! }
//! 
//! #[derive(FromEnv, Debug)]
//! struct DatabaseConfig {
//!     #[env("DB_URL", required)]
//!     url: String,
//!     
//!     #[env("DB_MAX_CONNECTIONS", default = "10")]
//!     max_connections: u32,
//! }
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AppConfig::load(std::path::Path::new(".env"))?;
//!     
//!     println!("App: {}", config.name);
//!     println!("Server: {}:{}", config.server.host, config.server.port);
//!     println!("Database: {}", config.database.url);
//!     
//!     Ok(())
//! }
//! ```

// Re-export all core functionality
pub use cfgloader_core::*;

// Re-export derive macro when derive feature is enabled
#[cfg(feature = "derive")]
pub use cfgloader_rs_macros::FromEnv;
