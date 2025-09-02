use crate::commands::main::prompt_to_open;
use crate::fetch::versions::get_versions;
use dialoguer::{theme::ColorfulTheme, Select};
use std::env;
use std::path::PathBuf;

pub fn list_files(is_audio: &bool) {
    let path: PathBuf = match env::current_dir() {
        Ok(s) => s,
        Err(_) => {
            panic!("No current dir found");
        }
    };
    let versions = get_versions(&path.as_path(), &is_audio);

    if versions.len() == 0 {
        eprintln!("No versions found");
        return;
    }

    let display: Vec<String> = versions
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let name = entry.file_name().to_string_lossy();
            format!("V{} - {}", versions.iter().count() - i, name)
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a version")
        .items(&display)
        .default(0)
        .interact()
        .ok(); // handle user cancel

    match &selection {
        Some(s) => {
            let selected_entry = &versions[*s];
            let _ = prompt_to_open(selected_entry.path(), &is_audio);
        }
        None => return,
    }
}
