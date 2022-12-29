use crate::colored::Colorize;
use serde::Deserialize;
use sqlite::Connection;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::Command,
};

#[derive(Deserialize, Debug)]
struct Config {
    loc: String,
}

use tempfile::NamedTempFile;
pub fn create_task(title: &String) {
    let conn = get_conn();

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(format!("Task: {}\n", title).as_bytes())
        .expect("Failed to write to temp file");
    let path = temp_file.path();

    Command::new("vim")
        .arg(path)
        .status()
        .expect("Failed to open Vim");
    let mut file = File::open(path).expect("Failed to open Temp File again");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read to string");

    conn.execute(format!(
        "INSERT INTO tasks (title, description) VALUES (\"{}\", \"{}\")",
        title, content
    ))
    .expect("Failed to insert task");

}

pub fn info(id: &i32) {
    let conn = get_conn();
    let mut content = String::new();
    for _ in conn.iterate(format!("select * from tasks where id = {}", id), |row| {
        content = format!("Task: {}\n{}", row[1].1.unwrap(), row[3].1.unwrap_or(""));
        true
    }) {}

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(content.as_bytes())
        .expect("Failed to write to temp file");

    let path = temp_file.path();

    Command::new("less")
        .arg(path)
        .status()
        .expect("Failed to open less");

}

pub fn update_task(id: &i32, title: &Option<String>) {
    let conn = get_conn();
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");

    // If title, update title too
    if let Some(title) = title {
        conn.execute(format!(
            "update tasks set title = \"{}\" where id = {}",
            title, id
        ))
        .expect("Failed to update title");
    }

    let mut content = String::new();
    for _ in conn.iterate(format!("select * from tasks where id = {}", id), |row| {
        content = format!("Task: {}\n{}", row[1].1.unwrap(), row[3].1.unwrap_or(""));
        true
    }) {}

    temp_file
        .write_all(content.as_bytes())
        .expect("Failed to write to temp file");

    let path = temp_file.path();

    Command::new("vim")
        .arg(path)
        .status()
        .expect("Failed to open Vim");

    let mut file = File::open(path).expect("Failed to open Temp File again");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read to string");

    conn.execute(format!(
        "update tasks set description = \"{}\" where id = {}",
        content, id
    ))
    .expect("Failed to update description");

}

pub fn toggle_task(id: &i32) {
    let conn = get_conn();
    // Get task from database, convert to boolean, and toggle it
    let mut done = false;
    for _ in conn.iterate(format!("select done from tasks where id = {}", id), |row| {
        match row[0].1 {
            Some("1") => done = true,
            _ => done = false,
        }
        true
    }) {}

    conn.execute(format!(
        "update tasks set done = {} where id = {}",
        !done as i32, id
    ))
    .expect("Failed to update task");
}

pub fn display_task() {
    let conn = get_conn();

    for _ in conn.iterate("select * from tasks;", |row| {
        match row[2].1 {
            Some("1") => {
                println!(
                    "- [x] ({}) {}",
                    row[0].1.unwrap(),
                    row[1].1.unwrap().green().bold()
                )
            }

            _ => {
                println!("- [ ] ({}) {}", row[0].1.unwrap(), row[1].1.unwrap().blue())
            }
        }
        true
    }) {}
}

pub fn delete_task(id: &i32) {
    let conn = get_conn();
    conn.execute(format!(
        "delete from
    tasks where id = {}",
        id
    ))
    .expect("Failed to delete task");
}

pub fn widget_loc(loc: &String) {
    // Let the location in $CONFIG_DIR/task/config.json
    let mut config_dir = dirs::config_dir().expect("Failed to get config directory");
    config_dir.push("task");
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    config_dir.push("config.json");
    let mut file = File::create(config_dir).expect("Failed to create config file");
    file.write_all(format!("{{\"loc\": \"{}\"}}", loc).as_bytes())
        .expect("Failed to write to config file");
}

pub fn widget(id: &i32) {
    // Get WidgetLoc, if it does not exist, Error, else, set $WIDGET_LOC/data.json {'task': $TASK}
    let mut config_dir = dirs::config_dir().expect("Failed to get config directory");
    config_dir.push("task/config.json");
    let mut file = File::open(config_dir).expect("Failed to open config file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read to string");

    let config: Config = serde_json::from_str(&content).expect("Failed to parse config file");

    let mut widget_dir = PathBuf::from(&config.loc);
    widget_dir.push("data.json");

    let mut file = File::create(widget_dir).expect("Failed to create widget file");

    let conn = get_conn();
    let mut content = String::new();
    for _ in conn.iterate(format!("select * from tasks where id = {}", id), |row| {
        content = row[1].1.unwrap().to_string();
        true
    }) {}

    file.write_all(format!("{{\"task\": \"{}\"}}", content).as_bytes())
        .expect("Failed to write to widget file");

}
fn get_conn() -> Connection {
    let mut config_dir = dirs::config_dir().expect("Failed to get config directory");
    config_dir.push("task/tasks.db");
    Connection::open(config_dir).expect("Failed to open database")
}
