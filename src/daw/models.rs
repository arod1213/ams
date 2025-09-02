#[derive(Debug, PartialEq, Eq)]
pub struct BounceFolders {
    pub contains: Vec<String>,
    pub excludes: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Daw {
    pub name: String,
    pub extension: String,
    pub backup_folders: Vec<String>,
    pub bounce_folders: BounceFolders,
}

pub fn fetch_daws() -> [Daw; 4] {
    let ableton: Daw = Daw {
        name: String::from("Ableton"),
        extension: String::from("als"),
        backup_folders: vec![String::from("Backup")],
        bounce_folders: BounceFolders {
            contains: vec![],
            excludes: vec![
                String::from("Samples"),
                String::from("Ableton Project Info"),
                String::from("Stems"),
            ],
        },
    };
    let reaper: Daw = Daw {
        name: String::from("Reaper"),
        extension: String::from("RPP"),
        backup_folders: vec![String::from("Backups")],
        bounce_folders: BounceFolders {
            contains: vec![],
            excludes: vec![String::from("Media")],
        },
    };

    let pro_tools: Daw = Daw {
        name: String::from("Pro Tools"),
        extension: String::from("ptx"),
        backup_folders: vec![String::from("Session File Backups")],
        bounce_folders: BounceFolders {
            contains: vec![String::from("Bounces")],
            excludes: vec![String::from("Stems"), String::from("Audio Files")],
        },
    };
    let logic: Daw = Daw {
        name: String::from("Logic"),
        extension: String::from("logicx"),
        backup_folders: vec![],
        bounce_folders: BounceFolders {
            contains: vec![String::from("Bounces")],
            excludes: vec![String::from("Stems")],
        },
    };

    [ableton, pro_tools, logic, reaper]
}
