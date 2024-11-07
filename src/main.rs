mod args;
use args::{AddSubCommands, Args, Commands};
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Add(add_cmd) => match add_cmd {
            AddSubCommands::Dir { key, value, tags } => println!("Add Dir {key}| {value}, {tags:?}"),
            AddSubCommands::Web { key, value, tags } => println!("Add Web {key}| {value}, {tags:?}"),
        },
    }
}
