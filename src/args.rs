use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Add(AddSubCommands),
}

#[derive(Subcommand)]
pub enum AddSubCommands {
    Dir {
        #[arg()]
        key: String,

        #[arg()]
        value: String,

        #[arg(short, long, use_value_delimiter = true)]
        tags: Option<Vec<String>>,
    },
    Web {
        #[arg()]
        key: String,

        #[arg()]
        value: String,

        #[arg(short, long, use_value_delimiter = true)]
        tags: Option<Vec<String>>,
    },
}
