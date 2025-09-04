use crate::models::{Daw, is_audio, is_bounce};
use crate::sort::sort_by_date;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn valid_audio(entry: DirEntry) -> Option<DirEntry> {
    let file_path = entry.path();
    if !is_audio(&file_path) {
        return None;
    }
    if is_bounce(&file_path) {
        return Some(entry);
    };
    None
}
fn valid_session(entry: DirEntry, show_backups: &bool) -> Option<DirEntry> {
    let file_path = entry.path();
    let Some(ext) = file_path.extension() else {
        return None;
    };
    let ext = ext.to_str().unwrap();
    match Daw::from_extension(&ext) {
        Some(v) => {
            if v.is_backup(file_path) && !show_backups {
                return None;
            }
            return Some(entry);
        }
        None => None,
    }
}

pub fn get_versions(path: &Path, is_audio: &bool, show_backups: &bool) -> Vec<DirEntry> {
    let mut versions: Vec<DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            let Ok(entry) = entry else {
                return None;
            };
            match is_audio {
                true => valid_audio(entry),
                false => valid_session(entry, &show_backups),
            }
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
