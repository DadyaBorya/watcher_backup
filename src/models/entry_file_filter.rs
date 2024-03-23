use std::usize;

use serde::Serialize;

#[derive(Default, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryFileFilter {
    pub regex: String,
    pub content: String,
    pub deep: Option<i8>,
}
