mod modules;

use clap::Parser;
use modules::args::Args;
use modules::clean::clean_repo;
use modules::init::init_repo;
use modules::stage::stage_file;
use modules::status::show_status;
use std;

fn main() {
    let args = Args::parse();

    match (
        args.stage_file,
        args.init_repo,
        args.clean_repo,
        args.repo_status,
    ) {
        (Some(stagef), false, false, false) => {
            if let Err(e) = stage_file(&stagef) {
                eprintln!("ovc: couldn't stage file. {}", e);
                std::process::exit(1);
            }
        }
        (None, false, true, false) => {
            if let Err(e) = clean_repo() {
                eprintln!("ovc: failed to clean up repository. {}", e);
                std::process::exit(1);
            }
        }
        (None, true, false, false) => {
            if let Err(e) = init_repo() {
                eprintln!("ovc: failed to initialize repository. {}", e);
                std::process::exit(1);
            }
        }
        (None, false, false, true) => {
            if let Err(e) = show_status() {
                eprintln!("ovc: failed to show status. {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            println!("Inappropriate usage. --help for help.");
        }
    }
}
