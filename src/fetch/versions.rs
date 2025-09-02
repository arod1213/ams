use crate::daw::sessions::is_backup;
use crate::daw::{audio::is_bounce, models::fetch_daws};
use crate::sort::sort_by_date;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn get_versions(path: &Path, is_audio: &bool) -> Vec<DirEntry> {
    let daws = fetch_daws();
    let mut versions: Vec<DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            let Ok(file) = entry else {
                return None;
            };
            let file_path = file.path();
            match is_audio {
                true => {
                    if is_bounce(&file_path, &daws) {
                        return Some(file);
                    };
                    return None;
                }
                false => {
                    if is_backup(&file_path, &daws) {
                        return None;
                    }
                    return Some(file);
                }
            }
        })
        .collect();

    sort_by_date(&mut versions);
    versions
}

fn index_vec(vec: &Vec<DirEntry>, num: isize) -> Option<DirEntry> {
    let index = if num >= 0 {
        num as usize
    } else {
        let len = vec.len() as isize;
        (len + num) as usize
    };

    if index.abs_diff(0) < vec.iter().count() {
        return Some(vec[index].to_owned());
    }
    return None;
}

pub fn get_version(versions: &Vec<DirEntry>, version_num: isize) -> Option<DirEntry> {
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
