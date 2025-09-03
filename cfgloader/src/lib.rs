// Re-export all core functionality
pub use core::*;

// Re-export derive macro when derive feature is enabled
#[cfg(feature = "derive")]
pub use macros::FromEnv;
