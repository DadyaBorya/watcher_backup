use std::path::PathBuf;

use regex::Regex;

use crate::models::{
    dir_entry::DirEntry, entry_dir_file_priority::EntryDirFilePriority,
    entry_dir_priority::EntryDirPriority, entry_file_filter::EntryFileFilter,
    entry_file_priority::EntryFilePriority,
};

pub fn map(content: String) -> Result<Vec<DirEntry>, String> {
    let mut entries = Vec::new();

    for line in content.lines() {
        let mut entry = DirEntry::default();

        entry.selected = is_selected(line);

        let split = line.split(">").collect::<Vec<&str>>();

        if split.len() != 2 {
            return Err("Error parsing".to_string());
        }

        let path = path(split[0]);
        entry.path = path.clone();

        let str_path = path.as_path().display().to_string();

        entry.entry_file_filter = file_filter(split[1])?;
        entry.entry_dir_file_priority = dir_file_priority(split[1], str_path.clone())?;
        entry.entry_dir_priority = dir_priority(split[1], str_path.clone())?;
        entry.entry_file_priority = file_priority(split[1], str_path.clone())?;

        entries.push(entry);
    }

    if entries.is_empty() {
        return Err("Error parsing".to_string());
    }

    return Ok(entries);
}

fn is_selected(line: &str) -> bool {
    line.chars().last().unwrap_or_default().eq(&'s')
}

fn path(line: &str) -> PathBuf {
    PathBuf::from(line)
}

fn file_filter(line: &str) -> Result<Option<Vec<EntryFileFilter>>, String> {
    let regex = Regex::new(r"1\[([^]]*)\]").unwrap();
    let mut filters = Vec::new();

    if let Some(captures) = regex.captures(line) {
        if let Some(group) = captures.get(1) {
            let group = group.as_str();
            let regex = Regex::new(r"\{(.+?),\s*(\d*|\s*),*\s*(.*?)\}").unwrap();

            for cap in regex.captures_iter(group) {
                let mut filter = EntryFileFilter::default();
                if let Some(regex) = cap.get(1) {
                    filter.regex = regex.as_str().to_string();
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(deep) = cap.get(2) {
                    if let Ok(number) = deep.as_str().parse::<usize>() {
                        filter.deep = Some(number as i8);
                    } else {
                        filter.deep = None
                    }
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(content) = cap.get(3) {
                    filter.content = content.as_str().to_string();
                }


                filters.push(filter);
            }
        }
    }

    if filters.is_empty() {
        return Ok(None);
    }

    Ok(Some(filters))
}

fn dir_file_priority(
    line: &str,
    path: String,
) -> Result<Option<Vec<EntryDirFilePriority>>, String> {
    let regex = Regex::new(r"2\[([^]]*)\]").unwrap();
    let mut priorities = Vec::new();

    if let Some(captures) = regex.captures(line) {
        if let Some(group) = captures.get(1) {
            let group = group.as_str();
            let regex = Regex::new(r"\{(.+?),\s*(\d*|\s*),\s*(\d+),*\s*(.*?)\}").unwrap();

            for cap in regex.captures_iter(group) {
                let mut priority = EntryDirFilePriority::default();
                if let Some(regex) = cap.get(1) {
                    priority.regex = regex.as_str().to_string();
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(deep) = cap.get(2) {
                    if let Ok(number) = deep.as_str().parse::<usize>() {
                        priority.deep = Some(number as i8);
                    } else {
                        priority.deep = None
                    }
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(p) = cap.get(3) {
                    if let Ok(number) = p.as_str().parse::<usize>() {
                        priority.priority = number;
                    } else {
                        return Err("Error parsing".to_string());
                    }
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(content) = cap.get(4) {
                    priority.content = content.as_str().to_string();
                }

                priority.root = path.clone();

                priorities.push(priority);
            }
        }
    }

    if priorities.is_empty() {
        return Ok(None);
    }

    Ok(Some(priorities))
}

fn dir_priority(line: &str, path: String) -> Result<Option<Vec<EntryDirPriority>>, String> {
    let regex = Regex::new(r"3\[([^]]*)\]").unwrap();
    let mut priorities = Vec::new();

    if let Some(captures) = regex.captures(line) {
        if let Some(group) = captures.get(1) {
            let group = group.as_str();
            let regex = Regex::new(r"\{(.+?),\s*(\d*|\s*),\s*(\d+)\}").unwrap();

            for cap in regex.captures_iter(group) {
                let mut priority = EntryDirPriority::default();
                if let Some(regex) = cap.get(1) {
                    priority.regex = regex.as_str().to_string();
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(deep) = cap.get(2) {
                    if let Ok(number) = deep.as_str().parse::<usize>() {
                        priority.deep = Some(number as i8);
                    } else {
                        priority.deep = None;
                    }
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(p) = cap.get(3) {
                    if let Ok(number) = p.as_str().parse::<usize>() {
                        priority.priority = number;
                    } else {
                        return Err("Error parsing".to_string());
                    }
                } else {
                    return Err("Error parsing".to_string());
                }

                priority.root = path.clone();

                priorities.push(priority);
            }
        }
    }

    if priorities.is_empty() {
        return Ok(None);
    }

    Ok(Some(priorities))
}

fn file_priority(line: &str, path: String) -> Result<Option<Vec<EntryFilePriority>>, String> {
    let regex = Regex::new(r"4\[([^]]*)\]").unwrap();
    let mut priorities = Vec::new();

    if let Some(captures) = regex.captures(line) {
        if let Some(group) = captures.get(1) {
            let group = group.as_str();
            let regex = Regex::new(r"\{(\d+),*\s*(.*?)\}").unwrap();

            for cap in regex.captures_iter(group) {
                let mut priority = EntryFilePriority::default();

                if let Some(p) = cap.get(1) {
                    if let Ok(number) = p.as_str().parse::<usize>() {
                        priority.priority = number;
                    } else {
                        return Err("Error parsing".to_string());
                    }
                } else {
                    return Err("Error parsing".to_string());
                }

                if let Some(content) = cap.get(2) {
                    priority.content = content.as_str().to_string();
                }

                priority.root = path.clone();

                priorities.push(priority);
            }
        }
    }

    if priorities.is_empty() {
        return Ok(None);
    }

    Ok(Some(priorities))
}
