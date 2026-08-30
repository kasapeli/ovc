use std::{
    self,
    fs::{self, OpenOptions},
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
    let mut index_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(index_dir)?;

    writeln!(
        index_file,
        "{} {}",
        stagef.to_string_lossy(),
        stage_file_hash
    )?;

    println!(
        "ovc: staged {:?}",
        stage_file.file_name().unwrap_or_default()
    );
    println!("ovc: blob saved to objects/{}", &stage_file_hash[..8]);

    Ok(())
}
