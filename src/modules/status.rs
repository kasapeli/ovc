use std::{
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

    if no_change {
        println!("ovc: no changes have been made");
    }

    Ok(())
}
