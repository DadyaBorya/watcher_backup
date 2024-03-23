use serde::Serialize;

#[derive(Default, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryDirFilePriority {
    pub regex: String,
    pub content: String,
    pub priority: usize,
    pub deep: Option<i8>,
    pub root: String,
}
