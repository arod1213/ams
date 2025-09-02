use crate::utils::checks::{get_extension, is_backup};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn find_projects(path: &Path) -> Vec<DirEntry> {
    let valid_ext = ["als", "ptx", "logicx"];

    let projects: Vec<DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.as_ref().unwrap();
            let file_path = entry.path();

            let Some(ext) = get_extension(&file_path) else {
                return None;
            };

            if !(valid_ext.contains(&ext)) {
                return None;
            }

            if !(is_backup(&file_path)) {
                return Some(entry.clone());
            }
            None
        })
        .collect();

    projects
}

pub fn get_latest(projects: &Vec<DirEntry>) -> Option<&DirEntry> {
    projects
        .iter()
        .filter_map(|entry| {
            entry
                .metadata()
                .ok()
                .and_then(|meta| meta.modified().ok().map(|time| (entry, time)))
        })
        .max_by_key(|&(_, modified)| modified)
        .map(|(entry, _)| entry)
}
