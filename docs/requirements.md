# TaskFlow — Requirements Specification

## 1. Project Overview

TaskFlow is a cross-platform command-line application for managing a personal to-do list.  
The application will allow users to create, view, update, and delete tasks from the terminal.  
Tasks are stored persistently in a local JSON file so that the task list is preserved between runs.

---

## 2. Functional Requirements

### 2.1 Supported Commands

| Command | Description                                        | Example Usage             |
| ------- | -------------------------------------------------- | ------------------------- |
| add     | Adds a new task to the list.                       | `taskflow add "Buy milk"` |
| list    | Displays all tasks with IDs and completion status. | `taskflow list`           |
| done    | Marks a task as completed by ID.                   | `taskflow done 2`         |
| delete  | Removes a task from the list by ID.                | `taskflow delete 2`       |

### 2.2 Command Behavior

- `add`: Assigns a unique ID to each new task.
- `list`: Shows all tasks in the order they were added, with completion indicators.
- `done`: Updates the task’s status to completed.
- `delete`: Removes the specified task permanently.

---

## 3. Non-Functional Requirements

### 3.1 Performance

- All commands must execute within 200ms for up to 1,000 tasks on a standard consumer laptop.

### 3.2 Portability

- Must run on Linux, macOS, and Windows.
- Must work in standard terminal environments.

### 3.3 Persistence

- Data stored in a local JSON file named `tasks.json` by default.
- Storage location configurable via command-line flag (e.g., `--data-dir`).

### 3.4 Code Quality

- Must pass `cargo fmt` with no formatting changes required.
- Must pass `cargo clippy` with no warnings.
- Code must compile without errors on stable Rust.

---

## 4. Acceptance Criteria

- User can add, list, mark as done, and delete tasks without runtime errors.
- Tasks persist between runs in `tasks.json`.
- Application compiles and runs on Linux, macOS, and Windows.
- No warnings from `cargo clippy`.
- No changes suggested by `cargo fmt`.

---

## 5. Out of Scope

- Networking features (e.g., cloud sync, remote storage).
- Graphical User Interface (GUI).
- User authentication or multi-user support.

---

## 6. Assumptions

- Users have Rust installed (stable channel).
- Users run the program from a terminal or command prompt.
- Users have read/write access to the directory where `tasks.json` is stored.
