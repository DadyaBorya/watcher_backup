use std::path::PathBuf;


use serde::Serialize;

use super::{
    entry_file_filter::EntryFileFilter,
    entry_dir_file_priority::EntryDirFilePriority,
    entry_dir_priority::EntryDirPriority,
    entry_file_priority::EntryFilePriority,
};


#[derive(Default, Clone, Debug, Serialize)]
pub struct DirEntry {
    pub path: PathBuf,
    pub children: Option<Vec<DirEntry>>,
    pub selected: bool,
    pub entry_file_filter: Option<Vec<EntryFileFilter>>,
    pub entry_dir_file_priority: Option<Vec<EntryDirFilePriority>>,
    pub entry_dir_priority: Option<Vec<EntryDirPriority>>,
    pub entry_file_priority: Option<Vec<EntryFilePriority>>,
}


impl DirEntry {
    pub fn path_to_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn path_depth(&self) -> usize {
        self.path.components().count()
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub fn short_name(&self) -> String {
        let file_name = self.path.file_name().and_then(|name| name.to_str());

        match file_name {
            Some(name) => name.to_string(),
            None => self.path.to_string_lossy().to_string(),
        }
    }

    pub fn is_just_selected(&self) -> bool {
        self.selected && self.entry_file_filter == None && self.entry_file_priority == None && self.entry_dir_file_priority == None
            && self.entry_dir_priority == None
    }
}
