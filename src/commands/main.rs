use dialoguer::Confirm;
use std::error::Error;
use std::path::Path;
use std::process::Command;

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
    let _ = Command::new("qlmanage")
        .arg("-p")
        .arg(path)
        // qlmanage -p blocks until you close preview, so spawn instead of output() if you want non-blocking:
        .spawn()?;
    // .output();
    Ok(())
}

pub fn prompt_to_open(path: &Path, is_audio: bool) -> Result<(), Box<dyn Error>> {
    match Confirm::new()
        .with_prompt("Would you like to open this file?")
        .interact()
        .unwrap()
    {
        false => return Ok(()),
        true => (),
    };

    match is_audio {
        true => {
            open_file_in_finder(path);
            let _ = preview_audio(path);
        }
        false => {
            open_file(path);
        }
    }

    Ok(())
}
