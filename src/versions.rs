use crate::sort::sort_by_date;
use std::{path::Path, time::Instant};
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

// skip large directories where files sessions should not be stored
fn should_skip(entry: &DirEntry) -> bool {
    let path = entry.path();
    let skip_dirs = ["/System", "/Applications", "/Library"];

    if skip_dirs.iter().any(|&x| path.starts_with(x)) {
        return true;
    }
    if path.starts_with("/Users") && path.to_str().unwrap().contains("Applications") {
        return true;
    }
    false
}

fn is_name_match(entry: &DirEntry, name: &Option<String>) -> bool {
    let Some(name) = name else { return true };

    if let Some(file_name) = entry.path().to_str() {
        if !file_name.to_lowercase().contains(&name.to_lowercase()) {
            return false;
        }
    }
    true
}

pub struct GetVersionInput<'a, F>
where
    F: Fn(DirEntry) -> Option<DirEntry>,
{
    pub path: &'a Path,
    pub f: F,
    pub name: Option<String>,
}

pub fn get_versions<F>(input: GetVersionInput<F>) -> Vec<DirEntry>
where
    F: Fn(DirEntry) -> Option<DirEntry>,
{
    let GetVersionInput { path, f, name } = input;
    let mut versions: Vec<DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| {
            if should_skip(&entry) | entry.path_is_symlink() | is_hidden(&entry) {
                return false;
            }
            true
        })
        .filter_map(|entry| {
            let Ok(entry) = entry else {
                return None;
            };
            if !is_name_match(&entry, &name) {
                return None;
            }
            f(entry)
        })
        .collect();

    sort_by_date(&mut versions);
    versions
}

fn index_vec<'a>(vec: &'a Vec<DirEntry>, num: isize) -> Option<&'a DirEntry> {
    let index = if num >= 0 {
        num as usize
    } else {
        let len = vec.len() as isize;
        (len + num) as usize
    };

    if index.abs_diff(0) < vec.iter().count() {
        return Some(&vec[index]);
    }
    return None;
}

pub fn get_version<'a>(versions: &'a Vec<DirEntry>, version_num: isize) -> Option<&'a DirEntry> {
    if versions.len() == 0 {
        eprintln!("No files found");
        return None;
    }

    let version = match index_vec(&versions, version_num) {
        Some(s) => s,
        None => {
            eprintln!("Invalid version number");
            return None;
        }
    };
    Some(version)
}
