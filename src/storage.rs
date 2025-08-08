use crate::task::Task;
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

#[cfg(test)]
mod tests {
    use crate::task::Task;
    use std::fs;

    #[test]
    fn test_save_and_load_tasks() {
        let test_file = "test_tasks.json";
        let tasks = vec![Task {
            id: 1,
            description: "Test".to_string(),
            done: false,
        }];
        // Save
        {
            let file = std::fs::File::create(test_file).unwrap();
            serde_json::to_writer_pretty(file, &tasks).unwrap();
        }
        // Load
        {
            let file = std::fs::File::open(test_file).unwrap();
            let loaded: Vec<Task> = serde_json::from_reader(file).unwrap();
            assert_eq!(loaded.len(), 1);
            assert_eq!(loaded[0].description, "Test");
        }
        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}
