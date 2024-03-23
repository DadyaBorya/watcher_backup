use serde::Serialize;

#[derive(Default, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryDirPriority {
    pub regex: String,
    pub deep: Option<i8>,
    pub priority: usize,
    pub root: String,
}

