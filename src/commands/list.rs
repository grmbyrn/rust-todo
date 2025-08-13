use crate::task::TaskManager;

pub fn run_list(manager: &mut TaskManager) {
    for task in manager.list_tasks() {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{} {} - {}", status, task.id, task.description);
    }
}
