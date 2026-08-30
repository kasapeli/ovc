use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(short = 'i', long = "init")]
    pub init_repo: bool,

    #[arg(short = 'C', long = "CLEAN")]
    pub clean_repo: bool,

    #[arg(short = 's', long = "stage", value_name = "FILE")]
    pub stage_file: Option<PathBuf>,

    #[arg(short = 'S', long = "status")]
    pub repo_status: bool,

    #[arg(short = 'c', long = "commit", value_name = "MESSAGE")]
    pub message: String,
}
