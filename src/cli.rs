use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new task
    New {
        /// Title for the task
        #[clap(short, long)]
        title: String,
    },

    /// Updates an already existing task, i.e opens it in vim
    Update {
        /// Title of the task
        #[clap(short, long)]
        id: i32,

        #[clap(short, long)]
        title: Option<String>,
    },

    /// Deletes an existing task
    Delete {
        /// Title of the task to delete
        #[clap(short, long)]
        id: i32,
    },

    /// Lists all exising tasks
    List {},

    Toggle {
        /// ID of the task to toggle
        #[clap(short, long)]
        id: i32,
    },

    Widget {
        /// ID of the task to put into the widget
        #[clap(short, long)]
        id: i32,
    },

    Info {
        /// ID of the task to get info about
        #[clap(short, long)]
        id: i32,
    },
}
