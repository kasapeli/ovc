use sha256::{digest, try_digest};
use std::fs::{self};
use std::io::{self};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_commit(message: String) -> io::Result<()> {
    let index_path = Path::new(".ovc/index");
    let head_path = Path::new(".ovc/HEAD");
    let objects_dir = Path::new(".ovc/objects");

    if !index_path.exists() {
        println!("ovc: not a repository");
        return Ok(());
    }

    let index_metadata = fs::metadata(index_path)?;
    if index_metadata.len() == 0 {
        println!("ovc: nothing to commit");
        return Ok(());
    }

    let tree_hash = try_digest(index_path)?;

    let parent_hash = if head_path.exists() {
        fs::read_to_string(head_path)?.trim().to_string()
    } else {
        "None".to_string()
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let commit_content = format!(
        "tree {}\nparent {}\ntimestamp {}\n\n{}\n",
        tree_hash, parent_hash, timestamp, message
    );

    let commit_hash = digest(commit_content.clone());

    let commit_object_path = objects_dir.join(&commit_hash);
    fs::write(&commit_object_path, commit_content)?;

    let mut perms = fs::metadata(&commit_object_path)?.permissions();
    perms.set_readonly(true);
    fs::set_permissions(&commit_object_path, perms)?;

    fs::write(head_path, &commit_hash)?;
    fs::write(index_path, "")?;

    println!("ovc: commit created successfully");
    println!("ovc: [{}] {}", &commit_hash[..8], message);

    Ok(())
}
