use std::path::Path;

pub fn get_extension(path: &Path) -> Option<&str> {
    let ext = match path.extension() {
        Some(s) => s,
        None => return None
    };
    ext.to_str()
}
