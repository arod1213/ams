use std::fs;
use std::path::Path;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn find_best_match(path: &Path, name: &str, is_dir: &bool) -> Option<fs::DirEntry> {
    let matcher = SkimMatcherV2::default();
    let mut best_match: Option<(fs::DirEntry, i64)> = None;

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return None,
    };

    for file in entries {
        let entry = match file {
            Ok(s) => s,
            Err(_) => continue,
        };

        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if file_type.is_dir() != *is_dir {
            continue;
        }

        let file_name = match entry.file_name().to_str() {
            Some(s) => s.to_lowercase(),
            None => continue,
        };

        let score = match matcher.fuzzy_match(&file_name, &name.to_lowercase()) {
            Some(s) => s,
            None => continue,
        };

        match &best_match {
            Some((_, best_score)) if score > *best_score => {
                best_match = Some((entry, score));
            }
            None => {
                best_match = Some((entry, score));
            }
            _ => {}
        }
    }

    best_match.map(|(entry, _)| entry)
}
