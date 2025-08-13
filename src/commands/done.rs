use crate::storage::save_tasks;
use crate::task::TaskManager;

pub fn run_done(manager: &mut TaskManager, id: u32) {
    match manager.mark_done(id) {
        Ok(_) => {
            save_tasks(&manager.tasks).unwrap();
            println!("Task {} marked as done.", id);
        }
        Err(e) => eprintln!("{}", e),
    }
}
