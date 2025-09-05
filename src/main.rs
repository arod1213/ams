mod args;
mod commands;
mod models;
mod sort;
mod versions;

use clap::Parser;

use args::{Args, Command};
use dotenv::dotenv;
use walkdir::DirEntry;

use crate::{
    commands::{list::list_files, open::open_file},
    models::{Daw, is_audio, is_bounce},
};

fn valid_audio(entry: DirEntry, show_all: bool) -> Option<DirEntry> {
    let file_path = entry.path();
    if !is_audio(&file_path) {
        return None;
    }
    if is_bounce(&file_path) || show_all {
        return Some(entry);
    };
    None
}

fn valid_session(entry: DirEntry, show_backups: bool) -> Option<DirEntry> {
    let file_path = entry.path();
    let ext = file_path.extension()?.to_str()?;

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

fn main() {
    dotenv().ok();
    let args = Args::parse();

    match args.command {
        Command::Open {
            version: version_num,
            audio,
        } => {
            if audio {
                open_file(version_num, |x| valid_audio(x, false));
            } else {
                open_file(version_num, |x| valid_session(x, false));
            }
        }
        Command::List {
            audio,
            backups,
            name,
        } => {
            if audio {
                list_files(|x| valid_audio(x, backups), name);
            } else {
                list_files(|x| valid_session(x, backups), name);
            }
        }
    };
}
