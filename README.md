# Task Manager

A simple command-line **Task Manager** built in **Rust** to help you organize tasks efficiently. This application lets you add, list, complete, and delete tasks with optional priorities (low, medium, high). Tasks are saved to a `tasks.json` file, ensuring they persist even after the program exits.

## Features

- **Add Tasks**: Create tasks with a description and optional priority.
- **List Tasks**: View all tasks with their ID, description, priority, and status (pending/completed).
- **Complete Tasks**: Mark tasks as completed by their ID.
- **Delete Tasks**: Remove tasks from the list by their ID.
- **Persistence**: Tasks are stored in `tasks.json` for retention across sessions.
- **User-Friendly CLI**: Powered by the `clap` crate for easy command parsing.

## Prerequisites

Before running the project, ensure you have:

- **Rust and Cargo**: Install via `rustup` from [rust-lang.org](https://www.rust-lang.org/tools/install).
- A terminal (e.g., Command Prompt, PowerShell, Bash, or any Linux terminal).
- **Git**: To clone the repository (optional).

## Installation

Follow these steps to set up the project:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/cyberne7work/rust-to-do.git
   cd rust-to-do
   ```

2. **Install Dependencies**:
   The project uses these Rust crates (listed in `Cargo.toml`):
   - `clap`: For command-line argument parsing.
   - `serde` and `serde_json`: For JSON serialization to save/load tasks.
   
   Run the following to download dependencies and build the project:
   ```bash
   cargo build
   ```

## Usage

Run the program using `cargo run` followed by a subcommand. Below are the available commands with examples:

- **Add a Task**:
  ```bash
  cargo run -- add "Buy groceries" medium
  ```
  - Adds a task with description `"Buy groceries"` and priority `medium` (optional: `low`, `medium`, `high`).
  - Example output:
    ```
    Buy groceries
    Added task with ID 1
    ```

- **List All Tasks**:
  ```bash
  cargo run -- list
  ```
  - Displays all tasks with details.
  - Example output:
    ```
    ID: 1, Description: Buy groceries, Priority: Some(Medium), Status: Pending
    ```
  - If no tasks exist:
    ```
    No tasks available.
    ```

- **Complete a Task**:
  ```bash
  cargo run -- complete 1
  ```
  - Marks the task with ID `1` as completed.
  - Example output:
    ```
    Task 1 marked as completed
    ```

- **Delete a Task**:
  ```bash
  cargo run -- delete 1
  ```
  - Deletes the task with ID `1`.
  - Example output:
    ```
    Task 1 deleted
    ```

**Note**: Tasks are automatically saved to `tasks.json` after each command and loaded when the program starts, ensuring persistence.

## Project Structure

```
task-manager/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   ├── main.rs         # Entry point, initializes TaskManager and handles CLI
│   ├── task.rs         # Task and TaskManager structs, persistence logic
│   ├── cli.rs          # CLI parsing and command execution
├── tasks.json          # Stores tasks (created after first run)
├── README.md           # Project documentation (this file)
├── LICENSE             # MIT License file
```

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add your feature"
   ```
4. Push to the branch:
   ```bash
   git push origin feature/your-feature
   ```
5. Open a Pull Request on GitHub.

Please use `cargo fmt` for code formatting and include tests where applicable.

## License

This project is licensed under the [MIT License](LICENSE). See the LICENSE file for details.

## Acknowledgments

- Built while learning Rust from *The Rust Programming Language* book.
- Thanks to the Rust community and crates like `clap` and `serde` for making development easier.