use crate::storage::save_tasks;
use crate::task::TaskManager;

pub fn run_undone(manager: &mut TaskManager, id: u32) {
    match manager.mark_undone(id) {
        Ok(_) => {
            save_tasks(&manager.tasks).unwrap();
            println!("Task {} marked as not done.", id);
        }
        Err(e) => eprintln!("{}", e),
    }
}
