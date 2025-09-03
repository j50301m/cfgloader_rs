//! # CFGLoader Example
//!
//! This example demonstrates how to use the cfgloader library to load configuration
//! from environment variables and .env files with automatic type conversion and validation.

use std::env;

use cfgloader_rs::*;

/// Main configuration structure that combines all application settings
///
/// The `FromEnv` derive macro automatically generates code to load configuration
/// from environment variables and .env files.
#[allow(dead_code)]
#[derive(FromEnv, Debug)]
struct Config {
    /// Database URL with a default value
    ///
    /// Usage: Set `DB_URL=postgresql://localhost/mydb` in environment or .env file
    /// If not set, defaults to "sqlite://test.db"
    #[env("DB_URL", default = "sqlite://test.db")]
    db_url: String,
    /// Nested configuration struct - will automatically call App::load()
    /// Fields without #[env] annotations are treated as nested configurations
    app: App,
}

/// Application-specific configuration
///
/// Demonstrates various types of environment variable configurations:
/// - Required fields (will fail if not provided)
/// - Optional fields with defaults
/// - Array/vector parsing with custom separators
#[allow(dead_code)]
#[derive(FromEnv, Debug)]
struct App {
    /// Server port - optional field that defaults to empty string if not set
    ///
    /// Usage: Set `PORT=8080` in environment
    /// Note: No default specified, so it will be empty string if not provided
    #[env("PORT")]
    pub port: String,

    /// Application name - REQUIRED field
    ///
    /// Usage: Set `APP_NAME=MyApplication` in environment or .env file
    /// This field is required and the program will fail if it's not provided
    #[env("APP_NAME", required)]
    pub app_name: String,

    /// Feature flags as a vector of strings
    ///
    /// Usage: Set `FEATURES=auth,logging,metrics` in environment
    /// The `split = ","` parameter tells the parser to split the string by commas
    /// Defaults to ["foo", "bar"] if not provided
    #[env("FEATURES", default = "foo,bar", split = ",")]
    pub features: Vec<String>,

    /// Another nested configuration struct
    other: OtherSettings,
}

/// Additional settings to demonstrate nested configuration loading
///
/// This struct shows how you can organize configuration into logical groups
/// and still load everything automatically with a single call.
#[derive(FromEnv, Debug)]
#[allow(dead_code)]
struct OtherSettings {
    /// First setting with default value
    ///
    /// Usage: Set `OTHER_SETTING_1=custom_value` in environment
    #[env("OTHER_SETTING_1", default = "default_value_1")]
    pub setting_1: String,

    /// Second setting with default value
    ///
    /// Usage: Set `OTHER_SETTING_2=another_value` in environment
    #[env("OTHER_SETTING_2", default = "default_value_2")]
    pub setting_2: String,
}

fn main() {
    let dotenv_path = std::path::Path::new(".env");
    let config = Config::load(dotenv_path).unwrap();

    println!("{:#?}", config);
}