use crate::daw::models::Daw;
use crate::utils::file::get_extension;
use std::path::Path;

fn is_audio(path: &Path) -> bool {
    let ext = match get_extension(path) {
        Some(s) => s,
        None => return false,
    };

    let audio_ext = ["wav", "mp3", "aiff", "aif"];
    audio_ext.contains(&ext)
}

pub fn is_bounce(path: &Path, daws: &[Daw]) -> bool {
    if !is_audio(path) {
        return false;
    }

    {
        if path.to_str().unwrap().to_lowercase().contains("stem") {
            return false;
        };
    }

    let path_name = path.to_str().unwrap().to_lowercase();
    !daws.iter().any(|d| {
        d.bounce_folders
            .excludes
            .iter()
            .any(|b| path_name.contains(&b.to_lowercase()))
    })
}
