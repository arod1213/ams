use crate::{daw::models::Daw, file::get::get_extension};
use std::path::Path;

pub fn is_backup(path: &Path, daws: &[Daw]) -> bool {
    let Some(ext) = get_extension(path) else {
        return true;
    };

    let parent_dir = match path.parent() {
        Some(s) => s,
        None => return false,
    };
    let parent_name = parent_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();

    if !daws.iter().any(|v| v.extension == ext) {
        return true;
    };

    daws.iter().any(|v| {
        v.backup_folders
            .iter()
            .any(|b| parent_name == b.to_lowercase())
    })
}

#[cfg(test)]
mod tests {
    use crate::daw::models::fetch_daws;

    use super::*;

    #[test]
    pub fn test_backup() {
        let path = Path::new("/aidan/Bounces/Backup/project.cub");
        let daws = fetch_daws();

        println!("daws {:#?}", daws);

        let back: Vec<String> = daws.iter().flat_map(|d| d.backup_folders.clone()).collect();
        println!("daws {:#?}", back);
        assert!(is_backup(&path, &daws));
    }
}
