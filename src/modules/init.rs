use std::{self, fs, io, path::Path};

pub fn init_repo() -> io::Result<()> {
    let ovc_dir = Path::new(".ovc");

    if ovc_dir.exists() {
        println!("ovc: already a repository");
        return Ok(());
    }

    fs::create_dir(ovc_dir)?;

    let objects_dir = ovc_dir.join("objects");
    fs::create_dir(objects_dir)?;

    let index_file = ovc_dir.join("index");
    fs::File::create(index_file)?;

    println!(
        "ovc: initialized a repository in {:?}",
        ovc_dir.canonicalize()?
    );

    Ok(())
}
