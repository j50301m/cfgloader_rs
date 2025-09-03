# CFGLoader 項目重構說明

## 重構目標

將原來的 workspace 結構重新組織，提供統一的單包依賴接口，並排除 example 項目。

## 新的項目結構

```
cfgloader/
├── Cargo.toml                    # Workspace 配置
├── cfgloader-core/               # 核心功能庫 (普通 crate)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                # 包含 trait、error、工具函數
├── cfgloader-macros/             # Derive macro 庫 (proc-macro crate)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                # 包含 #[derive(FromEnv)] macro
├── cfgloader/                    # 統一接口庫 (普通 crate)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                # 重新導出所有功能
└── example/                      # 示例項目 (被排除在 workspace 外)
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## 主要變更

### 1. Workspace 配置

- 將根目錄設為 workspace
- 成員：`cfgloader-core`、`cfgloader-macros`、`cfgloader`
- 排除：`example`
- 使用 `resolver = "2"`

### 2. cfgloader-core

- **類型**：普通 library crate
- **包含**：
  - `FromEnv` trait
  - `CfgError` error 類型
  - 工具函數：`get_env`、`parse_scalar`、`parse_vec`
  - `fallback` 模組
- **依賴**：`dotenvy`、`thiserror`

### 3. cfgloader-macros

- **類型**：proc-macro crate
- **包含**：
  - `#[derive(FromEnv)]` procedural macro
  - 屬性解析邏輯
  - 代碼生成邏輯
- **依賴**：`cfgloader-core`、`proc-macro2`、`syn`、`quote`

### 4. cfgloader (主要用戶接口)

- **類型**：普通 library crate
- **包含**：
  - 重新導出 `cfgloader-core` 的所有功能
  - 可選的 derive macro 功能 (通過 feature 控制)
- **Features**：
  - `default = ["derive"]`：默認啟用 derive macro
  - `derive`：啟用 derive macro 功能
- **依賴**：`cfgloader-core`、可選的 `cfgloader-macros`

### 5. Example 使用方式

**✨ 現在用戶只需要依賴一個包！**

```toml
[dependencies]
cfgloader = { path = "../cfgloader" }
```

```rust
use cfgloader::*;

#[derive(FromEnv, Debug)]
struct Config {
    #[env("DB_URL", default = "sqlite://test.db")]
    db_url: String,
}
```

## 優勢

1. **單包依賴**：用戶只需要依賴 `cfgloader` 一個包即可使用所有功能
2. **清晰分離**：內部架構仍然保持核心功能與 macro 分離
3. **Feature 控制**：用戶可以通過 feature 控制是否啟用 derive macro
4. **避免 proc-macro 限制**：通過分層架構避免了 proc-macro crate 的導出限制
5. **向後兼容**：保持了所有原有功能

## Feature 控制選項

```toml
# 默認使用 (包含 derive macro)
cfgloader = "0.1.0"

# 只使用核心功能，不包含 derive macro
cfgloader = { version = "0.1.0", default-features = false }

# 明確啟用 derive 功能
cfgloader = { version = "0.1.0", features = ["derive"] }
```

## 測試驗證

✅ 項目構建成功  
✅ 單包依賴正常工作  
✅ Example 運行正常  
✅ 環境變數解析功能正常  
✅ 巢狀結構載入正常  
✅ 所有原有功能保持不變  
✅ Feature 控制功能正常  

這次重構成功地實現了單包依賴的目標，用戶現在只需要依賴 `cfgloader` 一個包就能使用所有功能，大大簡化了使用體驗。
