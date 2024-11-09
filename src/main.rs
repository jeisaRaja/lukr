mod args;
mod commands;
mod database;
use args::{AddSubCommands, Args, Commands, ListType};
use clap::Parser;
use commands::{add_dir_bookmark, add_web_bookmark};
use database::{check_db_exist, create_db, select_bookmark};

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
                add_dir_bookmark(db_path, key, value, tags.unwrap_or(vec![])).unwrap()
            }
            AddSubCommands::Web { key, value, tags } => {
                add_web_bookmark(db_path, key, value, tags.unwrap_or(vec![])).unwrap()
            }
        },
        Commands::Dir { key, tags: _ } => {
            let bookmark = select_bookmark(db_path, &key, ListType::Dir);
            println!("{bookmark:?}")
        }
        Commands::Web { key, tags: _ } => {
            let bookmark = select_bookmark(db_path, &key, ListType::Web);
            println!("{bookmark:?}")
        }
        Commands::List { tags: _, item_type } => println!("List {item_type:?}"),
    }
}
