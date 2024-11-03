use std::path::PathBuf;

mod rustmarks;
use clap::{Parser, Subcommand};
use rustmarks::{add_bookmark, list_bookmarks, remove_bookmark, update_bookmark};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Add a bookmark
    Add {
        /// The path of the bookmark
        path: String,

        /// The name of the bookmark
        #[arg(short, long)]
        name: Option<String>,

        /// The description of the bookmark
        #[arg(short, long)]
        description: Option<String>,
    },

    // Remove a bookmark
    Remove {
        /// The id of the bookmark
        id: i32,
    },

    Update {
        /// The id of the bookmark
        id: i32,

        /// The path of the bookmark
        #[arg(short, long)]
        path: Option<String>,

        /// The name of the bookmark
        #[arg(short, long)]
        name: Option<String>,

        /// The description of the bookmark
        #[arg(short, long)]
        description: Option<String>,
    },

    // List all bookmarks
    List {},

    // Print the command for selected bookmark
    Command {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add {
            name,
            path,
            description,
        }) => add_bookmark(name, path, description),
        Some(Commands::Remove { id }) => remove_bookmark(id),
        Some(Commands::List {}) => list_bookmarks(),
        Some(Commands::Command {}) => {
            rustmarks::print_command();
        }
        Some(Commands::Update {
            id,
            path,
            name,
            description,
        }) => {
            update_bookmark(id, name, path, description);
        }
        None => {
            rustmarks::main();
        }
    }
}
