use clap::{Parser, Subcommand};

/// TaskFlow CLI - Manage your tasks from terminal
#[derive(Parser)]
#[command(name = "taskflow")]
#[command(about = "A simple Rust CLI todo app", long_about = None)]
/// Represents the command-line interface for the todo app.
/// Parses user commands and arguments using clap.
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
    /// Mark a task as not done by ID
    Undone {
        /// ID of the task
        id: u32,
    },
    /// Delete a task by ID
    Delete {
        /// ID of the task
        id: u32,
    },
}
