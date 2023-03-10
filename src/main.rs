use clap::Parser;

mod cli;
use cli::{Cli, Commands::*};
use sqlite::Connection;
mod task;
use task::*;

extern crate colored;

fn main() {
    let _ = init();
    let cli = Cli::parse();
    match &cli.command {
        Some(New { title }) => create_task(&title),
        Some(Update { id, title }) => update_task(id, title),
        Some(List {}) => display_task(),
        Some(Toggle { id }) => toggle_task(id),
        Some(Info { id }) => info(id),
        Some(Delete { id }) => delete_task(id),
        Some(WidgetLoc { loc }) => widget_loc(loc),
        Some(Widget { id }) => widget(id),
        _ => {
            println!("\nInvalid Command, Run `task help` for help with using the CLI")
        }
    };
}

fn init() {
    let mut config_dir = dirs::config_dir().expect("Failed to get config directory");
    config_dir.push("task");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }
    config_dir.push("tasks.db");
    if !config_dir.exists() {
        std::fs::File::create(&config_dir).expect("Failed to create database file");
    }
    let conn = Connection::open(config_dir).expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, done INTEGER default 0 not null, description TEXT)",
    )
    .expect("Failed to create table");
}
