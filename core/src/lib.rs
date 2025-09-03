use std::env;
use thiserror::Error;

#[doc(hidden)]
pub mod fallback {
    pub fn load_or_default<T>(_env_path: &std::path::Path) -> Result<T, crate::CfgError>
    where
        T: Default,
    {
        // We need a way to detect if T implements FromEnv
        // Due to Rust limitations, we use a simple approach: try to call T::load directly
        // If compilation fails, it means T doesn't implement FromEnv, so we use Default

        // Since we can't detect trait implementation at runtime, we return Default
        // Users need to explicitly use #[env(...)] to load environment variables
        Ok(T::default())
    }
}

#[derive(Debug, Error)]
pub enum CfgError {
    #[error("missing required env: {0}")]
    MissingEnv(&'static str),

    #[error("failed to parse env {key} value `{value}` into {ty}: {source}")]
    ParseError {
        key: &'static str,
        value: String,
        ty: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("failed to load env: {msg}: {source}")]
    LoadError {
        msg: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    }
}

pub trait FromEnv: Sized {
    fn load(env_path: &std::path::Path) -> Result<Self, CfgError>;
}

/// Utility function for macros: read env and return Option<String>
pub fn get_env(key: &'static str) -> Option<String> {
    env::var(key).ok()
}

/// Utility function for macros: load .env file
pub fn load_env_file(env_path: &std::path::Path) -> Result<(), CfgError> {
    // Try to load .env file if it exists, but don't fail if it doesn't
    match dotenvy::from_path(env_path) {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(CfgError::LoadError {
            msg: "failed to load .env file",
            source: Box::new(e),
        }),
    }
}

/// Utility function for macros: parse string to T
pub fn parse_scalar<T: std::str::FromStr>(
    key: &'static str,
    raw: String,
) -> Result<T, CfgError>
where
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    raw.parse::<T>().map_err(|e| CfgError::ParseError {
        key,
        value: raw,
        ty: std::any::type_name::<T>(),
        source: Box::new(e),
    })
}

/// Split string and parse each part to Vec<T>
pub fn parse_vec<T: std::str::FromStr>(
    key: &'static str,
    raw: String,
    sep: &'static str,
) -> Result<Vec<T>, CfgError>
where
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    if sep.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for part in raw.split(sep) {
        let s = part.trim().to_string();
        if s.is_empty() {
            continue;
        }
        out.push(parse_scalar::<T>(key, s)?);
    }
    Ok(out)
}
