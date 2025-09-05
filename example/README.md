# CFGLoader Example

This example demonstrates how to use the `cfgloader_rs` library to load configuration from environment variables and `.env` files.

## Running the Example

1. Create a `.env` file in this directory:
```bash
# .env
DB_URL=postgresql://localhost/myapp
PORT=3000
APP_NAME=MyAwesomeApp
FEATURES=auth,logging,metrics
OTHER_SETTING_1=custom_value
OTHER_SETTING_2=another_value
```

2. Run the example:
```bash
cargo run
```

## What it demonstrates

- **Required fields**: `APP_NAME` must be provided or the app will fail
- **Optional fields with defaults**: `PORT` defaults to empty string if not set
- **Nested configuration**: Multiple configuration structs can be composed
- **Array parsing**: `FEATURES` is parsed as comma-separated values into `Vec<String>`
- **Type conversion**: Automatic parsing of environment variables into appropriate types

## Configuration Structure

The example shows a three-level configuration structure:

```rust
Config {
    db_url: String,           // From DB_URL env var
    app: App {
        port: String,         // From PORT env var
        app_name: String,     // From APP_NAME env var (required)
        features: Vec<String>, // From FEATURES env var (comma-separated)
        other: OtherSettings {
            setting_1: String, // From OTHER_SETTING_1 env var
            setting_2: String, // From OTHER_SETTING_2 env var
        }
    }
}
```

## Try It Yourself

Modify the `.env` file or set environment variables to see how the configuration loading works:

```bash
export APP_NAME="Test App"
export FEATURES="auth,cache,monitoring"
cargo run
```

For more information, see the main [cfgloader_rs documentation](https://docs.rs/cfgloader_rs).
