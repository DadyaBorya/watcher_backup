use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::models::config::Config;
use crate::services::file_service;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Protocol {
    Https,
    Http,
    Webdav,
}

impl Protocol {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Cloud {
    Mega,
    GoogleDrive,
}

impl Cloud {
    pub fn name(&self, config_path: &PathBuf) -> String {
        let config_json = file_service::read_file(config_path).unwrap();
        let config: Config = serde_json::from_str(&config_json).unwrap();

        match self {
            Cloud::Mega => config.clouds.mega,
            Cloud::GoogleDrive => config.clouds.google_drive,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Scheduler {
    pub name: String,
    pub cron: String,
    pub speed: f32,
    pub clouds: HashMap<Cloud, Vec<Protocol>>,
    pub root: String,
}