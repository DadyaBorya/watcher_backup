use serde::Serialize;

#[derive(Default, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryFilePriority {
    pub content: String,
    pub priority: usize,
    pub root: String,
}
