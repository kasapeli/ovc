use std::{
    self, fs,
    io::{self, Write},
    path::Path,
};

use sha256::try_digest;

pub fn stage_file(stage_file: &Path) -> io::Result<()> {
    let stagef = stage_file.canonicalize()?;
    let stage_file_hash = try_digest(&stagef)?;

    let objects_dir = Path::new(".ovc/objects");
    let object_bak_dir = objects_dir.join(&stage_file_hash);

    let file_bytes = fs::read(&stagef)?;
    fs::write(&object_bak_dir, file_bytes)?;

    let index_dir = Path::new(".ovc/index");
    let abs_path_str = stagef.to_string_lossy().to_string();
    let mut index_lines = Vec::new();
    let mut file_already_tracked = false;

    if index_dir.exists() {
        let content = fs::read_to_string(index_dir)?;
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            if parts[0] == abs_path_str {
                index_lines.push(format!("{} {}", abs_path_str, stage_file_hash));
                file_already_tracked = true;
            } else {
                index_lines.push(line.to_string());
            }
        }
    }

    if !file_already_tracked {
        index_lines.push(format!("{} {}", abs_path_str, stage_file_hash));
    }

    fs::write(index_dir, index_lines.join("\n") + "\n")?;

    println!(
        "ovc: staged {:?}",
        stage_file.file_name().unwrap_or_default()
    );
    println!("ovc: blob saved to objects/{}", &stage_file_hash[..8]);

    Ok(())
}
