# CFGLoader RS 🚀

[![Crates.io](https://img.shields.io/crates/v/cfgloader_rs.svg)](https://crates.io/crates/cfgloader_rs)
[![Documentation](https://docs.rs/cfgloader_rs/badge.svg)](https://docs.rs/cfgloader_rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/YOUR_USERNAME/cfgloader_rs#license)

A simple, powerful, and ergonomic configuration loading library for Rust applications. CFGLoader automatically loads configuration from environment variables and `.env` files with compile-time validation and type safety.

## ✨ Features

- **🔧 Simple Setup**: Just derive `FromEnv` on your structs
- **🏗️ Type Safe**: Compile-time validation and automatic type conversion
- **📁 .env Support**: Automatic loading from `.env` files with `dotenvy` integration
- **🎯 Flexible**: Support for required fields, defaults, and custom parsing
- **📊 Array Support**: Parse comma-separated values into `Vec<T>`
- **🔗 Nested Configs**: Organize configuration into logical groups
- **🛡️ Error Handling**: Descriptive error messages for missing or invalid values
- **🚀 Zero Dependencies**: Minimal dependency footprint (only `dotenvy` and `thiserror`)

## 🚀 Quick Start

Add CFGLoader to your `Cargo.toml`:

```toml
[dependencies]
cfgloader_rs = "0.1"
```

### Basic Usage

```rust
use cfgloader_rs::*;

#[derive(FromEnv, Debug)]
struct Config {
    #[env("DATABASE_URL", default = "sqlite://app.db")]
    database_url: String,
    
    #[env("PORT", default = "8080")]
    port: u16,
    
    #[env("API_KEY", required)]
    api_key: String,
    
    #[env("FEATURES", default = "auth,logging", split = ",")]
    features: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load(std::path::Path::new(".env"))?;
    println!("Config: {:#?}", config);
    Ok(())
}
```

### Environment Variables

```bash
# .env file or environment variables
DATABASE_URL=postgresql://localhost/myapp
PORT=3000
API_KEY=your-secret-key
FEATURES=auth,logging,metrics,cache
```

## 📚 Examples

### Nested Configuration

```rust
use cfgloader_rs::*;

#[derive(FromEnv, Debug)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(FromEnv, Debug)]
struct ServerConfig {
    #[env("SERVER_HOST", default = "127.0.0.1")]
    host: String,
    
    #[env("SERVER_PORT", default = "8080")]
    port: u16,
}

#[derive(FromEnv, Debug)]
struct DatabaseConfig {
    #[env("DB_URL", required)]
    url: String,
    
    #[env("DB_MAX_CONNECTIONS", default = "10")]
    max_connections: u32,
}
```

### Array Configuration

```rust
use cfgloader_rs::*;

#[derive(FromEnv, Debug)]
struct Config {
    // Parse comma-separated values
    #[env("ALLOWED_HOSTS", default = "localhost,127.0.0.1", split = ",")]
    allowed_hosts: Vec<String>,
    
    // Parse numbers
    #[env("WORKER_THREADS", default = "1,2,4,8", split = ",")]
    worker_threads: Vec<u32>,
    
    // Custom separator
    #[env("TAGS", default = "web|api|service", split = "|")]
    tags: Vec<String>,
}
```

### Optional vs Required Fields

```rust
use cfgloader_rs::*;

#[derive(FromEnv, Debug)]
struct Config {
    // Required - will fail if not provided
    #[env("API_KEY", required)]
    api_key: String,
    
    // Optional with default
    #[env("DEBUG_MODE", default = "false")]
    debug_mode: bool,
    
    // Optional without default (uses type's Default implementation)
    #[env("OPTIONAL_SETTING")]
    optional_setting: String, // Will be empty string if not set
}
```

## 📖 Attribute Reference

### `#[env("ENV_VAR_NAME")]`

Load value from the specified environment variable.

```rust
#[env("PORT")]
port: u16,
```

### `#[env("ENV_VAR_NAME", default = "value")]`

Provide a default value if the environment variable is not set.

```rust
#[env("HOST", default = "127.0.0.1")]
host: String,
```

### `#[env("ENV_VAR_NAME", required)]`

Mark a field as required. The application will fail to start if this environment variable is not provided.

```rust
#[env("API_KEY", required)]
api_key: String,
```

### `#[env("ENV_VAR_NAME", split = "separator")]`

Parse the environment variable as a delimited string and convert to `Vec<T>`.

```rust
#[env("FEATURES", default = "auth,logging", split = ",")]
features: Vec<String>,
```

### Nested Structs

Fields without `#[env]` attributes are treated as nested configuration structs:

```rust
#[derive(FromEnv)]
struct Config {
    #[env("APP_NAME")]
    name: String,
    
    // This will call DatabaseConfig::load()
    database: DatabaseConfig,
}
```

## 🎯 Supported Types

CFGLoader supports any type that implements `FromStr`:

- **Primitives**: `String`, `bool`, `i32`, `u32`, `f64`, etc.
- **Collections**: `Vec<T>` where `T: FromStr`
- **Custom Types**: Any type implementing `FromStr`

```rust
use std::str::FromStr;

#[derive(Debug)]
struct LogLevel(String);

impl FromStr for LogLevel {
    type Err = std::convert::Infallible;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LogLevel(s.to_uppercase()))
    }
}

#[derive(FromEnv, Debug)]
struct Config {
    #[env("LOG_LEVEL", default = "info")]
    log_level: LogLevel,
}
```

## 🔧 Error Handling

CFGLoader provides descriptive error messages:

```rust
use cfgloader_rs::*;

#[derive(FromEnv)]
struct Config {
    #[env("PORT", required)]
    port: u16,
}

fn main() {
    match Config::load(std::path::Path::new(".env")) {
        Ok(config) => println!("Config loaded successfully!"),
        Err(CfgError::MissingEnv(var)) => {
            eprintln!("Missing required environment variable: {}", var);
        }
        Err(CfgError::ParseError { key, value, ty, source }) => {
            eprintln!("Failed to parse {} value '{}' as {}: {}", key, value, ty, source);
        }
        Err(e) => eprintln!("Configuration error: {}", e),
    }
}
```

## 🏗️ Architecture

CFGLoader consists of three main crates:

- **`cfgloader`**: Main crate that re-exports everything you need
- **`cfgloader-core`**: Core functionality and error types
- **`cfgloader-macros`**: Procedural macros for `FromEnv` derive

## 📝 License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🚀 Getting Started

Check out the [example](example/) directory for a complete working example, or run:

```bash
cd example
cargo run
```

For detailed API documentation, visit [docs.rs/cfgloader_rs](https://docs.rs/cfgloader_rs).
