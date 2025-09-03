use std::env;

use cfgloader::*;

#[derive(FromEnv,Debug)]
struct Config {
    #[env("DB_URL", default = "sqlite://test.db")]
    db_url: String,
    app : App,
}

#[derive(FromEnv,Debug)]
struct App {
    #[env("PORT", default = "8000")]
    pub port: String,
    #[env("APP_NAME",required)]
    pub app_name: String,
    #[env("FEATURES", default = "foo,bar", split = ",")]
    pub features: Vec<String>,
    other: OtherSettings,
}

#[derive(FromEnv,Debug)]
struct OtherSettings {
    #[env("OTHER_SETTING_1", default = "default_value_1")]
    pub setting_1: String,
    #[env("OTHER_SETTING_2", default = "default_value_2")]
    pub setting_2: String,
}


fn main() {
    let config = Config::load(std::path::Path::new("example/.env")).unwrap();
    println!("Config: {:#?}", config);
}