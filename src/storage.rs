use crate::task::Task;
use serde_json;
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter};

const STORAGE_FILE: &str = "tasks.json";

// Load tasks from the JSON file
pub fn load_tasks() -> io::Result<Vec<Task>> {
    let file = OpenOptions::new().read(true).open(STORAGE_FILE);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let tasks = serde_json::from_reader(reader).unwrap_or_default();
            Ok(tasks)
        }
        Err(_) => Ok(Vec::new()), // If file doesn't exist, return empty list
    }
}

// Save tasks to the JSON file
pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(STORAGE_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}
