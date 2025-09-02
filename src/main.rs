mod args;
mod commands;
mod daw;
mod sort;
mod utils;

use clap::Parser;

use args::{Args, Command};
use dotenv::dotenv;

use crate::commands::{list::list_files, open::open_file};

fn main() {
    dotenv().ok();
    let args = Args::parse();

    match args.command {
        Command::Open {
            version_num,
            bounce,
        } => {
            if bounce {
                open_file(version_num, &true);
            } else {
                open_file(version_num, &false);
            }
        }
        Command::List { bounce } => {
            if bounce {
                list_files(&true);
            } else {
                list_files(&false);
            }
        }
    };
}
