mod args;
mod commands;
mod database;
use args::{AddSubCommands, Args, Commands};
use clap::Parser;
use commands::add_web_bookmark;
use database::{check_db_exist, create_db};

pub type Error = Box<dyn std::error::Error>;

fn main() {
    let db_path = "./lukr.db";
    if !check_db_exist(db_path) {
        create_db(db_path).unwrap();
    }

    let args = Args::parse();

    match args.command {
        Commands::Add(add_cmd) => match add_cmd {
            AddSubCommands::Dir { key, value, tags } => {
                println!("Add Dir {key} | {value}, {tags:?}")
            }
            AddSubCommands::Web { key, value, tags } => {
                add_web_bookmark(db_path, key, value, tags.unwrap_or(vec![])).unwrap()
            }
        },
        Commands::Dir { key, tags } => println!("Dir {key} {tags:?}"),
        Commands::Web { key, tags } => println!("Web {key} {tags:?}"),
        Commands::List { value, tags: _ } => println!("List {value}"),
    }
}
