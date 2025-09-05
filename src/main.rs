mod args;
mod commands;
mod models;
mod sort;
mod versions;

use std::env;

use clap::Parser;

use args::{Args, Command};
use dotenv::dotenv;

use crate::commands::{list::list_files, open::open_file};

fn main() {
    dotenv().ok();
    let args = Args::parse();

    let Ok(_) = env::set_current_dir("/Users/aidanrodriguez") else {
        return;
    };

    match args.command {
        Command::Open {
            version: version_num,
            audio,
        } => {
            open_file(version_num, audio, false);
        }
        Command::List { audio, backups } => {
            list_files(audio, backups);
        }
    };
}
