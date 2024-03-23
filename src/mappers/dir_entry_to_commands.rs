use std::{collections::HashSet, fs, path::Path};
use std::path::PathBuf;

use regex::Regex;

use crate::{
    models::{command::Command, dir_entry::DirEntry, entry_file_filter::EntryFileFilter},
    services::file_service,
};
use crate::models::entry_dir_file_priority::EntryDirFilePriority;
use crate::models::entry_dir_priority::EntryDirPriority;
use crate::models::entry_file_priority::EntryFilePriority;

pub fn map(mut entries: Vec<DirEntry>) -> Result<Vec<Command>, String> {
    delete_not_exist_entries(&mut entries);

    add_file_filter(&mut entries);

    let mut commands: HashSet<Command> = HashSet::new();

    get_commands(&mut entries, &mut commands);

    let mut commands: Vec<Command> = commands.into_iter().collect();

    commands.sort_by_key(|cmd| cmd.priority.unwrap_or(usize::MAX));

    for cmd in &mut commands {
        let mut path = PathBuf::from(cmd.local_path.clone().replace(":", ""));
        path.pop();
        cmd.remote_path = path.to_string_lossy().to_string();
    }



    Ok(commands)
}

fn get_commands(entries: &mut Vec<DirEntry>, set: &mut HashSet<Command>) {
    while !entries.is_empty() {
        let mut entry = entries.pop().unwrap();

        if entry.is_file() {
            if let Some(command) = get_file_command(&mut entry) {
                insert_command(set, command);
            }
            continue;
        }

        let file_priorities = if let Some(priorities) = entry.entry_dir_file_priority.clone() {
            let mut new_priorities = vec![];
            for priority in priorities {
                if priority.deep == None {
                    new_priorities.push(priority);
                    continue;
                }

                if priority.deep.unwrap() == -1 {
                    continue;
                }

                new_priorities.push(EntryDirFilePriority {
                    regex: priority.regex,
                    content: priority.content,
                    priority: priority.priority,
                    deep: Some(priority.deep.unwrap() - 1),
                    root: priority.root,
                })
            }

            Some(new_priorities)
        } else {
            None
        };

        let dir_priority = if let Some(priorities) = entry.entry_dir_priority.clone() {
            let mut new_priorities = vec![];

            for priority in priorities {
                if priority.deep == None {
                    new_priorities.push(priority);
                    continue;
                }

                if priority.deep.unwrap() == -1 {
                    continue;
                }

                new_priorities.push(EntryDirPriority {
                    regex: priority.regex,
                    priority: priority.priority,
                    deep: Some(priority.deep.unwrap() - 1),
                    root: priority.root,
                })
            }

            Some(new_priorities)
        } else {
            None
        };

        let file_filter = if let Some(filters) = entry.entry_file_filter.clone() {
            let mut new_filters = vec![];

            for filter in filters {
                if filter.deep == None {
                    new_filters.push(filter);
                    continue;
                }

                if filter.deep.unwrap() == -1 {
                    continue;
                }

                new_filters.push(EntryFileFilter {
                    regex: filter.regex.to_owned(),
                    content: filter.content.to_owned(),
                    deep: Some(filter.deep.unwrap() - 1),
                })
            }

            Some(new_filters)
        } else {
            None
        };

        if entry.is_just_selected() {
            if let Ok(new_entries) = fs::read_dir(&entry.path) {
                for entry in new_entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let mut entry = DirEntry::default();
                        entry.path = path;
                        entry.selected = true;

                        if entry.is_file() {
                            if let Some(filters) = file_filter.as_ref() {
                                if !compare_file_by_filters(&entry, filters) {
                                    continue;
                                }
                            }

                            entries.push(entry)
                        }
                    }
                }
            }
            continue;
        }

        if let Ok(new_entries) = fs::read_dir(&entry.path) {
            for entry in new_entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let mut entry = DirEntry::default();
                    entry.path = path;

                    match entry.is_dir() {
                        true => {
                            entry.entry_file_filter = file_filter.clone();


                            if let Some(priorities) = dir_priority.as_ref() {
                                let mut new_dir_priorities = vec![];

                                for dir_priority in priorities {
                                    if compare_entry_name_by_regex(&entry.short_name(), &dir_priority.regex) {
                                        if let Ok(dir_entries) = fs::read_dir(&entry.path) {
                                            for dir_entry in dir_entries {
                                                if let Ok(dir_entry) = dir_entry {
                                                    let path = dir_entry.path();
                                                    let mut dir_entry = DirEntry::default();
                                                    dir_entry.path = path;

                                                    if dir_entry.is_file() {
                                                        dir_entry.entry_file_priority = Some(vec![EntryFilePriority {
                                                            content: "".to_string(),
                                                            priority: dir_priority.priority,
                                                            root: dir_priority.root.to_owned(),
                                                        }]);
                                                        entries.push(dir_entry);
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    new_dir_priorities.push(EntryDirPriority {
                                        regex: dir_priority.regex.to_owned(),
                                        deep: dir_priority.deep,
                                        priority: dir_priority.priority,
                                        root: dir_priority.root.to_owned(),
                                    })
                                }

                                if !new_dir_priorities.is_empty() {
                                    entry.entry_dir_priority = Some(new_dir_priorities);
                                }
                            }

                            if let Some(priorities) = file_priorities.as_ref() {
                                let mut new_dir_file_priorities = vec![];

                                for dir_file_priority in priorities {
                                    new_dir_file_priorities.push(EntryDirFilePriority {
                                        regex: dir_file_priority.regex.to_owned(),
                                        content: dir_file_priority.content.to_owned(),
                                        priority: dir_file_priority.priority,
                                        deep: dir_file_priority.deep,
                                        root: dir_file_priority.root.to_owned(),
                                    })
                                }

                                if !new_dir_file_priorities.is_empty() {
                                    entry.entry_dir_file_priority = Some(new_dir_file_priorities);
                                }
                            }

                            if entry.entry_dir_priority != None || entry.entry_dir_file_priority != None {
                                entries.push(entry);
                            }
                        }
                        false => {
                            if let Some(filters) = file_filter.as_ref() {
                                if !compare_file_by_filters(&entry, filters) {
                                    continue;
                                }
                            }

                            if let Some(priorities) = file_priorities.as_ref() {
                                let mut new_file_priorities = vec![];

                                for file_priority in priorities {
                                    if !compare_entry_name_by_regex(&entry.short_name(), &file_priority.regex) {
                                        continue;
                                    }

                                    new_file_priorities.push(EntryFilePriority {
                                        content: file_priority.content.to_owned(),
                                        priority: file_priority.priority,
                                        root: file_priority.root.to_owned(),
                                    })
                                }

                                if !new_file_priorities.is_empty() {
                                    entry.entry_file_priority = Some(new_file_priorities);
                                    entries.push(entry);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


fn get_file_command(entry: &mut DirEntry) -> Option<Command> {
    if !entry.selected && entry.entry_file_priority == None {
        return None;
    }

    let priority = get_file_priority_by_file_priority(entry);
    Some(Command {
        local_path: entry.path_to_string(),
        remote_path: "".to_string(),
        priority,
    })
}

fn get_file_priority_by_file_priority(entry: &DirEntry) -> Option<usize> {
    let mut min_list = vec![];

    if let Some(priorities) = entry.entry_file_priority.as_ref() {
        for priority in priorities {
            if priority.content == "" {
                min_list.push(priority.priority);
                continue;
            }

            match compare_file_content_by_regex(&entry.path, &priority.content) {
                true => min_list.push(priority.priority),
                _ => {}
            }
        }
    }

    return match min_list.is_empty() {
        true => None,
        false => min_list.iter().min().map(|&value| value),
    };
}

fn delete_not_exist_entries(entries: &mut Vec<DirEntry>) {
    entries.retain(|entry| entry.path.exists());
}

fn delete_entries_with_only_filter(entries: &mut Vec<DirEntry>) {
    entries.retain(|entry| {
        !(entry.entry_file_filter != None
            && entry.entry_dir_priority == None
            && entry.entry_file_priority == None
            && entry.entry_dir_file_priority == None
            && !entry.selected)
    });
}

fn add_file_filter(entries: &mut Vec<DirEntry>) {
    for entry in entries.clone().iter() {
        if let Some(filters) = entry.entry_file_filter.as_ref() {
            for sub_entry in entries.iter_mut() {
                if sub_entry.path == entry.path
                    || !sub_entry.path_to_string().contains(&entry.path_to_string())
                    || sub_entry.is_file()
                {
                    continue;
                }

                for filter in filters.iter() {
                    let entry_depth = entry.path_depth();
                    let sub_entry_depth = sub_entry.path_depth();

                    if let None = sub_entry.entry_file_filter {
                        sub_entry.entry_file_filter = Some(vec![]);
                    }

                    let sub_entry_filter = sub_entry.entry_file_filter.as_mut().unwrap();

                    let new_filter = match filter.deep {
                        Some(deep) => {
                            let deep = deep as usize;
                            if deep + entry_depth >= sub_entry_depth {
                                EntryFileFilter {
                                    regex: filter.regex.clone(),
                                    content: filter.content.clone(),
                                    deep: Some((entry_depth + deep - sub_entry_depth) as i8),
                                }
                            } else {
                                continue;
                            }
                        }
                        None => filter.clone(),
                    };

                    if !sub_entry_filter.contains(&new_filter) {
                        sub_entry_filter.push(new_filter);
                    }
                }
            }

            entries.retain(|sub_entry| {
                !(sub_entry.is_file() && !compare_file_by_filters(sub_entry, filters))
            });
        }
    }
    delete_entries_with_only_filter(entries);
}

fn compare_file_by_filters(entry: &DirEntry, filters: &Vec<EntryFileFilter>) -> bool {
    for filter in filters.iter() {
        if !compare_entry_name_by_regex(&entry.short_name(), &filter.regex) {
            continue;
        }

        if filter.content == "" {
            return true;
        }

        match compare_file_content_by_regex(&entry.path, &filter.content) {
            true => return true,
            _ => {}
        }
    }
    false
}

fn compare_entry_name_by_regex(name: &str, regex: &str) -> bool {
    let regex = Regex::new(regex).unwrap();

    return regex.is_match(name);
}

fn compare_file_content_by_regex(path: &Path, regex: &str) -> bool {
    if let Ok(content) = file_service::read_file(path) {
        let regex = Regex::new(regex).unwrap();

        return regex.is_match(&content);
    }

    false
}

fn insert_command(set: &mut HashSet<Command>, item: Command) {
    if let Some(command) = set.get(&item) {
        match (command.priority, &item.priority) {
            (None, Some(_)) => {
                set.remove(&item);
                set.insert(item);
            }
            (Some(old_priority), Some(new_priority)) => {
                if new_priority < &old_priority {
                    set.remove(&item);
                    set.insert(item);
                }
            }
            _ => {}
        }
    } else {
        set.insert(item);
    }
}