use clap::Parser;

mod cli;
use anyhow::Error;
use cli::{Cli, Commands::*};
use sqlite::Connection;
mod task;
use task::*;

fn main() {
    init();

    let _ = init();
    let cli = Cli::parse();
    match &cli.command {
        Some(New { title }) => create_task(&title),
        _ => {
            println!("\nInvalid Command, Run `ideabank help` for help with using the CLI")
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
        "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, done INTEGER, description TEXT)",
    )
    .expect("Failed to create table");
    drop(conn);
}
