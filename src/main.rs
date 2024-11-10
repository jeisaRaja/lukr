mod args;
mod commands;
mod database;
use std::fs;

use args::{AddSubCommands, Args, Commands, ListType};
use clap::Parser;
use commands::{add_dir_bookmark, add_web_bookmark};
use database::create_db;

pub type Error = Box<dyn std::error::Error>;

fn main() {
    let db_dir = dirs::home_dir()
        .unwrap()
        .join(".local")
        .join("share")
        .join("lukr");

    let db_path = db_dir.join("lukr.db");
    if !db_dir.exists() || !db_path.exists() {
        fs::create_dir_all(db_dir).expect("Failed to create database directory");
        create_db(&db_path).unwrap();
    }
    let db_path_str = db_path.to_string_lossy().to_string();

    let args = Args::parse();

    match args.command {
        Commands::Add(add_cmd) => match add_cmd {
            AddSubCommands::Dir { key, value, tags } => {
                add_dir_bookmark(&db_path_str, key, value, tags.unwrap_or(vec![])).unwrap()
            }
            AddSubCommands::Web { key, value, tags } => {
                add_web_bookmark(&db_path_str, key, value, tags.unwrap_or(vec![])).unwrap()
            }
        },
        Commands::Dir { key, tags: _ } => {
            let bookmark = commands::select_bookmark(&db_path_str, &key, ListType::Dir);
            println!("{bookmark}")
        }
        Commands::Web { key, tags: _ } => {
            let bookmark = commands::select_bookmark(&db_path_str, &key, ListType::Web);
            println!("{bookmark}")
        }
        Commands::List { tags: _, item_type } => println!("List {item_type:?}"),
    }
}
