use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
    path::Path,
};

use sha256::try_digest;

pub fn show_status() -> io::Result<()> {
    let index_dir = Path::new(".ovc/index");

    if !index_dir.exists() {
        println!("ovc: not a repository.");
        return Ok(());
    }

    let index_file = fs::File::open(&index_dir)?;
    let read = io::BufReader::new(index_file);

    let mut tracked_files = HashSet::new();
    let mut no_change = true;

    for change in read.lines() {
        let line = change?;

        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            continue;
        }

        let tracked_path = parts[0];
        let hash = parts[1];
        let watch = Path::new(tracked_path);

        tracked_files.insert(tracked_path.to_string());

        if !watch.exists() {
            println!("ovc: deleted {}", tracked_path);
            no_change = false;
        } else {
            match try_digest(watch) {
                Ok(nhash) => {
                    if nhash != hash {
                        println!("ovc: modified {}", tracked_path);
                        no_change = false;
                    }
                }
                Err(_) => {
                    println!("ovc: error reading {}", tracked_path);
                    no_change = false;
                }
            }
        }
    }

    let mut found_untracked = false;

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Ok(lpath) = path.canonicalize() {
                let abs_path = lpath.to_string_lossy().to_string();

                if !tracked_files.contains(&abs_path) {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                    println!("ovc: untracked {}", file_name);

                    found_untracked = true;
                    no_change = false;
                }
            }
        }
    }

    if !found_untracked {
        println!("ovc: no untracked files");
    }

    if no_change {
        println!("ovc: no changes have been made");
    }

    Ok(())
}
