mod cli;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use storage::{load_tasks, save_tasks};
use task::TaskManager;

fn main() {
    let cli = Cli::parse();
    let tasks = load_tasks().unwrap_or_else(|err| {
        eprintln!(
            "Warning: could not load tasks ({}), starting with an empty list.",
            err
        );
        Vec::new()
    });
    let mut manager = TaskManager { tasks };

    match &cli.command {
        Commands::Add { description } => {
            let task = manager.add_task(description.to_string());
            let task_id = task.id;
            let task_description = task.description.clone();
            save_tasks(&manager.tasks).unwrap();
            println!("Added task: {} - {}", task_id, task_description);
        }
        Commands::List => {
            for task in manager.list_tasks() {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{} {} - {}", status, task.id, task.description);
            }
        }
        Commands::Done { id } => match manager.mark_done(*id) {
            Ok(_) => {
                save_tasks(&manager.tasks).unwrap();
                println!("Task {} marked as done.", id);
            }
            Err(e) => eprintln!("{}", e),
        },
        Commands::Delete { id } => match manager.delete_task(*id) {
            Ok(_) => {
                save_tasks(&manager.tasks).unwrap();
                println!("Task {} deleted.", id);
            }
            Err(e) => eprintln!("{}", e),
        },
    }
}
