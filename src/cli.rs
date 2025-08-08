use clap::{Parser, Subcommand};

/// TaskFlow CLI - Manage your tasks from terminal
#[derive(Parser)]
#[command(name = "taskflow")]
#[command(about = "A simple Rust CLI todo app", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Description of the task
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as done by ID
    Done {
        /// ID of the task
        id: u32,
    },
    /// Delete a task by ID
    Delete {
        /// ID of the task
        id: u32,
    },
}
