# Config loader for Rust

[![Crates.io](https://img.shields.io/crates/v/cfgloader_rs.svg)](https://crates.io/crates/cfgloader_rs)
[![Documentation](https://docs.rs/cfgloader_rs/badge.svg)](https://docs.rs/cfgloader_rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/j50301m/cfgloader_rs#license)

A simple, powerful, and ergonomic configuration loading library for Rust applications. CFGLoader automatically loads configuration from environment variables and `.env` files with compile-time validation and type safety.

## ✨ Features

A wrapper around `dotenvy` that provides the `FromEnv` derive macro and utilities to simplify the complexity of reading environment variables.

- Simple derive macro for automatic configuration loading
- Type-safe parsing with compile-time validation
- Built-in support for required fields, defaults, and custom parsing
- Array support with configurable separators
- Nested configuration structures
- Descriptive error handling

## 🚀 Quick Start

Add `cfgloader_rs` to your `Cargo.toml`:

```toml
[dependencies]
cfgloader_rs = "1.0"
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
    // One path
    let config = Config::load(std::path::Path::new(".env"))?;
    // Or multiple paths
    let config2 = Config::load_iter(vec![std::path::Path::new(".env"),std::path::Path::new(".env.local")])?;
    println!("Config: {:#?}", config);
    Ok(())
}
```
### Multiple .env Fallback


You can use `load_iter` to try multiple .env files in order:

```rust
let config = Config::load_iter([
    std::path::Path::new(".env.local"),
    std::path::Path::new(".env"),
])?;
```
This will try `.env.local` first, then `.env` if the first is not found.

### Environment Variables
## 🧬 API Reference

```rust
pub trait FromEnv: Sized {
    fn load(env_path: &std::path::Path) -> Result<Self, CfgError>;
    fn load_iter<I, P>(paths: I) -> Result<Self, CfgError>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<std::path::Path>;
}
```

- `load(env_path: &Path)`: Load config from a single .env file
- `load_iter<I, P>(paths: I)`: Try multiple paths, return on first success

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

## 🔧 Attribute Reference

- `#[env("ENV_VAR_NAME")]` - Load value from the specified environment variable
- `#[env("ENV_VAR_NAME", default = "value")]` - Provide a default value if the environment variable is not set
- `#[env("ENV_VAR_NAME", required)]` - Mark a field as required. The application will fail to start if this environment variable is not provided
- `#[env("ENV_VAR_NAME", split = "separator")]` - Parse the environment variable as a delimited string and convert to `Vec<T>`
- Nested Structs - Fields without `#[env]` attributes are treated as nested configuration structs

## 🔤 Supported Types

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

## ⚠️ Error Handling

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

- **`cfgloader_rs`**: Main crate that re-exports everything you need
- **`cfgloader-core`**: Core functionality and error types
- **`cfgloader_rs_macros`**: Procedural macros for `FromEnv` derive

## 📄 License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

#### Development Setup

To contribute to this project, initialize the repository for development:

```bash
make init
```

This will:

- Install Git hooks that automatically run `cargo fmt`, `cargo clippy`, and tests before each push
- Install useful development tools (`cargo-audit`, `cargo-outdated`, `cargo-expand`)

You can run all CI checks manually with:

```bash
make ci          # Run all quality checks (fmt, clippy, check, test, doc)
make help        # Show available commands
```

## 🎯 Getting Started

Check out the [example](https://github.com/j50301m/cfgloader_rs/tree/main/example) directory for a complete working example, or run:

```bash
cd example
cargo run
```

For detailed API documentation, visit [docs.rs/cfgloader_rs](https://docs.rs/cfgloader_rs).
