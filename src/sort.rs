use std::time::SystemTime;
use walkdir::DirEntry;

fn get_modified_time(entry: &DirEntry) -> SystemTime {
    match entry.metadata() {
        Ok(metadata) => metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        Err(_) => SystemTime::UNIX_EPOCH,
    }
}

pub fn sort_by_date(entries: &mut Vec<DirEntry>) {
    entries.sort_by(|a, b| {
        let a_time = get_modified_time(a);
        let b_time = get_modified_time(b);

        b_time.cmp(&a_time) // newest first
    });
}
