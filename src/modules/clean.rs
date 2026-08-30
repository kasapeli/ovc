use std::{self, fs, io, path::Path};

pub fn clean_repo() -> io::Result<()> {
    let ovc_dir = Path::new(".ovc");

    if !ovc_dir.exists() {
        println!("ovc: not a repository");
        return Ok(());
    }

    println!(
        "ovc: cleaned up repository at {:?}",
        ovc_dir.canonicalize()?
    );

    fs::remove_dir_all(ovc_dir)?;

    Ok(())
}
