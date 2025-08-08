mod cli;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use task::TaskManager;

fn main() {
    let cli = Cli::parse();
    let mut manager = TaskManager::new();

    match &cli.command {
        Commands::Add { description } => {
            let task = manager.add_task(description.to_string());
            println!("Added task: {} - {}", task.id, task.description);
        }
        Commands::List => {
            for task in manager.list_tasks() {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{} {} - {}", status, task.id, task.description);
            }
        }
        Commands::Done { id } => match manager.mark_done(*id) {
            Ok(_) => println!("Task {} marked as done.", id),
            Err(e) => eprintln!("{}", e),
        },
        Commands::Delete { id } => match manager.delete_task(*id) {
            Ok(_) => println!("Task {} deleted.", id),
            Err(e) => eprintln!("{}", e),
        },
    }
}
