use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
    // pub fn protocols(&self) -> Vec<Protocol> {
    //     match self {
    //         Cloud::Mega => vec![Protocol::Https, Protocol::Http],
    //         Cloud::GoogleDrive => vec![Protocol::Https, Protocol::Http, Protocol::Webdav],
    //     }
    // }

    pub fn name(&self) -> String {
        match self {
            Cloud::Mega => String::from("mega"),
            Cloud::GoogleDrive => String::from("googledrive"),
        }
    }
    // pub fn list() -> Vec<Cloud> {
    //     vec![Cloud::Mega, Cloud::GoogleDrive]
    // }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Scheduler {
    pub name: String,
    pub cron: String,
    pub speed: f32,
    pub clouds: HashMap<Cloud, Vec<Protocol>>,
    pub root: String,
}