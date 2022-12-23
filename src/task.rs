use crate::colored::Colorize;
use sqlite::Connection;
use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};
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

    drop(conn);
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

    drop(conn);
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

    drop(conn);
}

fn get_conn() -> Connection {
    let mut config_dir = dirs::config_dir().expect("Failed to get config directory");
    config_dir.push("task/tasks.db");
    Connection::open(config_dir).expect("Failed to open database")
}
