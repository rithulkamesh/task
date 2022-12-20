use sqlite::Connection;
use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};
use tempfile::NamedTempFile;
pub fn create_task(title: &String) {
    let mut config_dir = dirs::config_dir().expect("Failed to get config directory");
    config_dir.push("task/tasks.db");
    let conn = Connection::open(config_dir).expect("Failed to open database");

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(format!("Task: {}\n", title).as_bytes())
        .unwrap();
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
