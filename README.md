# Rust Todo CLI App

A robust, well-structured command-line todo application written in Rust, following best practices for project organization, error handling, persistence, and testing.

---

## Features

- **Add tasks** with unique IDs and descriptions
- **List tasks** with status indicators ([x] for done, [ ] for not done)
- **Mark tasks as done** by ID
- **Delete tasks** by ID
- **Persistent storage** using a JSON file (`tasks.json`)
- **Comprehensive unit and integration tests**
- **Clear, user-friendly CLI interface** (powered by [clap](https://docs.rs/clap/))

---

## Usage

### Add a Task

```sh
cargo run -- add "Learn Rust"
```

### List All Tasks

```sh
cargo run -- list
```

### Mark a Task as Done

```sh
cargo run -- done <id>
```

### Delete a Task

```sh
cargo run -- delete <id>
```

---

## Project Structure

```
rust-todo/
├── src/
│   ├── main.rs        # CLI entry point and command wiring
│   ├── cli.rs         # CLI argument parsing (clap)
│   ├── task.rs        # Task and TaskManager logic
│   └── storage.rs     # Persistent storage (JSON)
├── tests/
│   └── cli.rs         # Integration tests for CLI
├── tasks.json         # Persistent task storage (auto-created)
├── Cargo.toml         # Dependencies and metadata
└── README.md
```

---

## Implementation Details

- **Error Handling:** Uses `anyhow` and graceful error messages; no panics on file I/O.
- **Persistence:** Tasks are loaded from `tasks.json` at startup and saved after every change.
- **Testing:**
  - Unit tests for all core logic (`add_task`, `list_tasks`, `mark_done`, `delete_task`, storage).
  - Integration tests for CLI commands using `assert_cmd` and `predicates`.
- **Formatting & Linting:** Passes `cargo fmt` and `cargo clippy` with no warnings.

---

## How to Run Tests

```sh
cargo test
```

- Runs all unit and integration tests.
- Ensures code correctness and CLI behavior.

---

## Extending

- Add more commands or features by updating `cli.rs` and wiring new logic in `main.rs`.
- Storage can be swapped for a database or other format by updating `storage.rs`.

---

## Dependencies

- [clap](https://docs.rs/clap/) (CLI parsing)
- [serde](https://docs.rs/serde/) & [serde_json](https://docs.rs/serde_json/) (serialization)
- [anyhow](https://docs.rs/anyhow/) (error handling)
- [assert_cmd](https://docs.rs/assert_cmd/) & [predicates](https://docs.rs/predicates/) (integration testing)

---

## Notes

- The app is cross-platform and works on any system with Rust installed.
- All code is idiomatic, modular, and documented for maintainability.

---

## License

This project is licensed under the [MIT License](LICENSE).
