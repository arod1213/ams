use walkdir::DirEntry;

use crate::commands::main::prompt_to_open;
use crate::versions::{GetVersionInput, get_version, get_versions};
use std::env;
use std::path::PathBuf;

pub fn open_file<F: Fn(DirEntry) -> Option<DirEntry>>(version_num: isize, f: F) {
    let path: PathBuf = match env::current_dir() {
        Ok(s) => s,
        Err(_) => {
            panic!("No current dir found");
        }
    };
    let input = GetVersionInput {
        path: &path.as_path(),
        f,
        name: None,
    };
    let versions = get_versions(input);

    let version = match get_version(&versions, version_num) {
        Some(s) => s,
        None => {
            return;
        }
    };

    let version_path = version.path();
    let version_name = version.file_name().to_str().unwrap();

    let num = if version_num >= 0 {
        versions.iter().count() as isize - version_num
    } else {
        version_num * -1
    };

    println!("Found: V{:?} {:?}", num, version_name);
    let _ = prompt_to_open(&version_path);
}
