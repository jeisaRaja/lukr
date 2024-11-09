use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ListType {
    Web,
    Dir,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Add(AddSubCommands),

    List {
        #[arg(short, long = "type", help = "Specify the type of item to list")]
        item_type: ListType,

        #[arg(short, long, use_value_delimiter = true)]
        tags: Option<Vec<String>>,
    },

    Dir {
        #[arg()]
        key: String,

        #[arg(short, long, use_value_delimiter = true)]
        tags: Option<Vec<String>>,
    },

    Web {
        #[arg()]
        key: String,

        #[arg(short, long, use_value_delimiter = true)]
        tags: Option<Vec<String>>,
    },
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
