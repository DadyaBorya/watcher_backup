use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CloudConfig {
    pub mega: String,
    pub google_drive: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppPath {
    pub watcher_backup: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub clouds: CloudConfig,
    pub paths: AppPath
}