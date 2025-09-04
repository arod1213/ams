use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, allow_hyphen_values = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Open {
        #[arg(short, long, default_value_t = 0)]
        version: isize,

        #[arg(short, long, default_value_t = false)]
        audio: bool,
    },

    List {
        #[arg(short, long, default_value_t = false)]
        audio: bool,

        #[arg(short, long, default_value_t = false)]
        backups: bool,
    },
}
