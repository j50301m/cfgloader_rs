use std::env;
use thiserror::Error;

#[doc(hidden)]
pub mod fallback {
    pub fn load_or_default<T>(_env_path: &std::path::Path) -> Result<T, crate::CfgError>
    where
        T: Default,
    {
        // 我們需要用一種方法來檢測 T 是否實現了 FromEnv
        // 由於 Rust 限制，我們使用一個簡單的方法：直接嘗試呼叫 T::load
        // 如果編譯失敗，則說明 T 沒有實現 FromEnv，我們就用 Default

        // 由於無法在運行時檢測 trait 實現，我們返回 Default
        // 用戶需要明確使用 #[env(...)] 來加載環境變數
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

/// 給巨集用的小工具：讀 env 並回傳 Option<String>
pub fn get_env(key: &'static str) -> Option<String> {
    env::var(key).ok()
}

/// 給巨集用的小工具：載入 .env 檔案
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

/// 給巨集用的小工具：解析字串為 T
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

/// 分割字串並逐一解析為 Vec<T>
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
