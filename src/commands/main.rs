use dialoguer::Confirm;
use std::error::Error;
use std::path::Path;
use std::process::Command;

use crate::models::is_audio;

fn open_file(path: &Path) {
    let status = Command::new("open")
        .arg(path)
        .status()
        .expect("Failed to open file");

    if status.success() {
        println!("Opened {:?}", path);
    } else {
        eprintln!("Failed to open {:?}", path);
    }
}

fn open_file_in_finder(path: &Path) {
    let status = Command::new("open")
        .arg("-R") // Reveals the file in Finder
        .arg(path)
        .status()
        .expect("Failed to open file in finder");

    if !(status.success()) {
        eprintln!("Failed to open {:?}", path);
    }
}

pub fn preview_audio(path: &Path) -> std::io::Result<()> {
    let _ = Command::new("open")
        .arg("-a")
        .arg("QuickTime Player")
        .arg(path)
        .spawn()?;
    // .output();
    Ok(())
}

pub fn prompt_to_open(path: &Path) -> Result<(), Box<dyn Error>> {
    match Confirm::new()
        .with_prompt("Would you like to open this file?")
        .interact()
        .unwrap()
    {
        false => return Ok(()),
        true => (),
    };

    open_file_in_finder(path);
    match is_audio(&path) {
        true => {
            let _ = preview_audio(path);
        }
        false => {
            open_file(path);
        }
    }

    Ok(())
}
