mod args;
mod commands;
mod models;
mod sort;
mod versions;

use clap::Parser;

use args::{Args, Command};
use dotenv::dotenv;

use crate::commands::{list::list_files, open::open_file};

fn main() {
    dotenv().ok();
    let args = Args::parse();

    match args.command {
        Command::Open { version_num, song } => {
            open_file(version_num, &song, &false);
            open_file(version_num, &song, &false);
        }
        Command::List { song, backups } => {
            list_files(&song, &backups);
        }
    };
}
