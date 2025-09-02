use crate::search::find_best_match;
use std::env;
use std::path::PathBuf;

fn set_path(name: &str, dir: &mut PathBuf) {
    if let Some(entry) = find_best_match(dir, name, &true) {
        *dir = entry.path().to_path_buf();
    } else {
        eprintln!("No match found with name {:?}", name);
    }
}

pub fn get_path(artist: &Option<String>, song: &Option<String>) -> PathBuf {
    let mut dir: PathBuf = match env::current_dir() {
        Ok(s) => s,
        Err(_) => {
            panic!("No current dir found");
        }
    };

    match &artist {
        Some(s) => set_path(&s, &mut dir),
        None => (),
    };
    match &song {
        Some(s) => set_path(&s, &mut dir),
        None => (),
    };

    dir
}
