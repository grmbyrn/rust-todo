mod cli;
mod commands;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use storage::load_tasks;
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
        Commands::Add { description } => commands::add::run_add(&mut manager, description),
        Commands::List => commands::list::run_list(&mut manager),
        Commands::Done { id } => commands::done::run_done(&mut manager, *id),
        Commands::Undone { id } => commands::undone::run_undone(&mut manager, *id),
        Commands::Delete { id } => commands::delete::run_delete(&mut manager, *id),
    }
}
