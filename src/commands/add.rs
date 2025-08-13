use crate::storage::save_tasks;
use crate::task::TaskManager;

pub fn run_add(manager: &mut TaskManager, description: &str) {
    let task = manager.add_task(description.to_string());
    let task_id = task.id;
    let task_description = task.description.clone();
    save_tasks(&manager.tasks).unwrap();
    println!("Added task: {} - {}", task_id, task_description);
}
