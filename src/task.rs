use serde::{Deserialize, Serialize};

/// Represents a single todo task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    /// Unique identifier for the task.
    pub id: u32,
    /// Description of the task.
    pub description: String,
    /// Whether the task is completed.
    pub done: bool,
}

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    /// Adds a new task with a unique ID.
    /// Returns a reference to the newly added task.
    pub fn add_task(&mut self, description: String) -> &Task {
        let new_id = self.next_id();
        let task = Task {
            id: new_id,
            description,
            done: false,
        };
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    // List all tasks
    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    // Mark a task as done by ID
    pub fn mark_done(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = true;
                Ok(())
            }
            None => Err(format!("Task with ID {} not found.", id)),
        }
    }

    /// Mark a task as not done by ID
    pub fn mark_undone(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                if !task.done {
                    return Err(format!("Task {} was not marked as done.", id));
                }
                task.done = false;
                Ok(())
            }
            None => Err(format!("Task with ID {} not found.", id)),
        }
    }

    // Delete a task by ID
    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            Ok(())
        } else {
            Err(format!("Task with id {} not found", id))
        }
    }

    // Helper: find the next available ID (always increment, never reuse)
    fn next_id(&self) -> u32 {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut manager = TaskManager { tasks: Vec::new() };
        let task = manager.add_task("Test task".to_string());
        assert_eq!(task.description, "Test task");
        assert!(!task.done);
        assert_eq!(task.id, 1);
    }

    #[test]
    fn test_list_tasks() {
        let mut manager = TaskManager { tasks: Vec::new() };
        manager.add_task("Task 1".to_string());
        manager.add_task("Task 2".to_string());
        let tasks = manager.list_tasks();
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_mark_done() {
        let mut manager = TaskManager { tasks: Vec::new() };
        manager.add_task("Task 1".to_string());
        assert!(manager.mark_done(1).is_ok());
        assert!(manager.list_tasks()[0].done);
        assert!(manager.mark_done(99).is_err());
    }

    #[test]
    fn test_mark_undone() {
        let mut manager = TaskManager { tasks: Vec::new() };
        manager.add_task("Task 1".to_string());
        manager.mark_done(1).unwrap();
        assert!(manager.mark_undone(1).is_ok());
        assert!(!manager.list_tasks()[0].done);
        assert!(manager.mark_undone(99).is_err());
    }

    #[test]
    fn test_delete_task() {
        let mut manager = TaskManager { tasks: Vec::new() };
        manager.add_task("Task 1".to_string());
        assert!(manager.delete_task(1).is_ok());
        assert_eq!(manager.list_tasks().len(), 0);
        assert!(manager.delete_task(99).is_err());
    }
}
