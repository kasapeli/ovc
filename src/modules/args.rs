use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Initialize repository
    #[arg(short = 'i', long = "init")]
    pub init_repo: bool,

    /// Uninitialize repository
    #[arg(long = "CLEAN")]
    pub clean_repo: bool,

    /// Stage file
    #[arg(short = 's', long = "stage", value_name = "FILE")]
    pub stage_file: Option<PathBuf>,

    /// Check status
    #[arg(short = 'S', long = "status")]
    pub repo_status: bool,

    /// Commit
    #[arg(short = 'c', long = "commit", value_name = "MESSAGE")]
    pub message: Option<String>,
}
