# TaskFlow — Design Document

## 1. Architecture Overview

The application is structured into the following modules for clear separation of concerns:

- **main.rs**  
  Entry point of the application. Responsible for initializing the CLI and dispatching commands to appropriate handlers.

- **cli.rs**  
  Handles command-line argument parsing using the `clap` crate. Translates user input into commands the program can execute.

- **task.rs**  
  Defines the `Task` struct and implements core task management logic (add, list, complete, delete).

- **storage.rs**  
  Manages persistent storage. Responsible for reading and writing task data to a JSON file.

---

## 2. Data Model

```rust
struct Task {
    id: u32,
    description: String,
    done: bool,
}

- `id`: Unique identifier for each task.

- `description`: Text describing the task.

- `done`: Boolean indicating completion status.
```

## 3. Flow Diagram

```plaintext
User Command
     ↓
CLI Parser (cli.rs)
     ↓
Task Logic (task.rs)
     ↓
Storage Layer (storage.rs)
     ↓
Output/Response
```

## 4. Error Handling Strategy

- Use Rust’s `Result` type to manage recoverable errors.

- All IO operations return `Result` to signal success or failure.

- Command parsing errors produce helpful messages and terminate gracefully.

## 5. Dependencies

- `clap` for CLI argument parsing.

- `serde` and `serde_json` for serialization of tasks.

- `anyhow` or custom error types for error handling (optional).

## 6. Module Interaction Summary

- main.rs initializes CLI and calls functions in cli.rs.

- cli.rs interprets commands, invoking methods in task.rs.

- task.rs uses storage.rs to persist task state.

## 7. Architecture Diagram

```
+--------------+       +----------+       +----------+       +------------+
|   User CLI   |  -->  |  cli.rs  |  -->  | task.rs  |  -->  | storage.rs |
+--------------+       +----------+       +----------+       +------------+
```
