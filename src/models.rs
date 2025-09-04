// list of extension (backup and main)
// names of backup folders
//
// Ableton (.als) -> (Backup)
// Pro tools (.ptx .bak.ptx, .ptf, .bak.ptf) -> (Session File Backups)
// Fl studio (.flp)
// Logic (.logicx, .logic)
// Reaper (.rpp, .rpp-bak) -> (Backups)
// Cubase (.cpr, .bak)
// Reason (.reason)

// Primary Extension
// Backup Extension
// Backup Folder(s)

// if not primary or backup extension -> remove
// if backups disabled, hide backup extensions / backup folders

use std::path::Path;

pub fn is_audio(path: &Path) -> bool {
    let audio_formats: [&str; 4] = ["wav", "mp3", "aif", "aiff"];
    let ext = match path.extension() {
        Some(v) => v.to_str().unwrap(),
        None => return false,
    };
    audio_formats.contains(&ext)
}

pub fn is_bounce(path: &Path) -> bool {
    let path_name = path.to_str().unwrap();
    if path_name.to_lowercase().contains("stem") {
        return false;
    }

    match path.file_stem() {
        Some(_) => (),
        None => return false,
    };

    let Some(parent) = path.parent().expect("no parent found").file_stem() else {
        return true;
    };
    let parent_low = parent.to_str().unwrap().to_lowercase();
    !(parent_low.contains("media")
        | parent_low.contains("audio files")
        | parent_low.contains("samples"))
}

pub enum Daw {
    Ableton,
    Logic,
    FlStudio,
    ProTools,
    Reaper,
    Reason,
    Cubase,
}

impl Daw {
    pub fn from_extension(ext: &str) -> Option<Self> {
        use Daw::*;
        let ext_low: &str = &ext.to_lowercase();
        match ext_low {
            "als" => Some(Ableton),
            "ptx" | "ptf" => Some(ProTools),
            "cpr" | "bak" => Some(Cubase),
            "reason" => Some(Reason),
            "rpp" | "rpp-bak" => Some(Reaper),
            "flp" => Some(FlStudio),
            "logicx" => Some(Logic),
            _ => None,
        }
    }

    pub fn is_backup(&self, path: &Path) -> bool {
        let path = path.to_str().unwrap();
        use Daw::*;
        match self {
            Ableton | Reaper | FlStudio => path.contains("Backup"),
            ProTools => path.contains(".bak") | path.contains("Session File Backup"),
            Cubase => path.contains(".bak"),
            _ => false,
        }
    }
}
