use std::collections::HashMap;
use std::path::Path;

pub fn get_extension(path: &Path) -> Option<&str> {
    let ext = match path.extension() {
        Some(s) => s,
        None => return None
    };
    ext.to_str()
}

pub fn is_backup(path: &Path) -> bool {
    let ext = get_extension(&path).unwrap();
    let parent_dir = match path.parent() {
        Some(s) => s.to_str().unwrap(),
        None => return false
    };
    let parent_path = Path::new(parent_dir);
    let parent_name = parent_path.file_name().unwrap();

    let mut backups: HashMap<&str, &str> = HashMap::new();
    backups.insert("als","Backup");
    backups.insert("ptx", "Session File Backups");

    if backups.contains_key(ext) {
        let backup_name = match backups.get(ext) {
            Some(s) => s,
            None => return false
        };
        if parent_name == *backup_name {
            return true
        }
        return false
    }
    return false
}
