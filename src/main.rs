use std::path::PathBuf;

mod rustmarks;
use clap::{Parser, Subcommand};

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
    List {
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        pathsonly: bool,
    },

    // Print the command for selected bookmark
    Command {},

    // Check if a bookmark exists
    Check {
        path: Option<String>
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add {
            name,
            path,
            description,
        }) => rustmarks::add_bookmark(name, path, description),
        Some(Commands::Remove { id }) => rustmarks::remove_bookmark(id),
        Some(Commands::List { pathsonly }) => rustmarks::list_bookmarks(pathsonly.clone()),
        Some(Commands::Command {}) => {
            rustmarks::print_command();
        }
        Some(Commands::Update {
            id,
            path,
            name,
            description,
        }) => {
            rustmarks::update_bookmark(id, name, path, description);
        }
        Some(Commands::Check { path }) => {
            rustmarks::check_bookmark(path);
        }
        None => {
            rustmarks::main();
        }
    }
}
