// src/main.rs
mod task;
mod cli;

use cli::{Cli, run_cli};
use task::TaskManager;
use clap::Parser;

fn main() {
    // Load tasks from file or start fresh
    let mut manager = TaskManager::load_from_file()
        .unwrap_or_else(|e| {
            println!("Error loading tasks: {}. Starting fresh.", e);
            TaskManager::new()
        });

    // Parse and execute the command
    let cli = Cli::parse();
    run_cli(&mut manager, cli);

    // Save tasks after the command
    manager.save_to_file().unwrap_or_else(|e| {
        println!("Error saving tasks: {}", e);
    });
}