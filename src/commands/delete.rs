use crate::storage::save_tasks;
use crate::task::TaskManager;

pub fn run_delete(manager: &mut TaskManager, id: u32) {
    match manager.delete_task(id) {
        Ok(_) => {
            save_tasks(&manager.tasks).unwrap();
            println!("Task {} deleted.", id);
        }
        Err(e) => eprintln!("{}", e),
    }
}
