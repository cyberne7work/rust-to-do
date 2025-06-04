// src/cli.rs
use crate::task::{Task, TaskManager, Priority};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Task Manager")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { description: String, priority: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
}

pub fn run_cli(manager: &mut TaskManager, cli: Cli) {
    match cli.command {
        Commands::Add { description, priority } => {
            println!("{}", description);
            let priority = priority.map(|p| match p.to_lowercase().as_str() {
                "low" => Priority::Low,
                "medium" => Priority::Medium,
                "high" => Priority::High,
                _ => Priority::Medium, // Default for invalid input
            });
            let id = manager.add_task(description, priority);
            println!("Added task with ID {}", id);
        }
        Commands::List => {
            let tasks = manager.list_tasks();
            if tasks.is_empty() {
                println!("No tasks available.");
                return;
            }
            for task in tasks {
                println!(
                    "ID: {}, Description: {}, Priority: {:?}, Status: {:?}",
                    task.id, task.description, task.priority, task.status
                );
            }
        }
        Commands::Complete { id } => match manager.complete_task(id) {
            Ok(()) => println!("Task {} marked as completed", id),
            Err(e) => println!("Error: {}", e),
        },
        Commands::Delete { id } => match manager.delete_task(id) {
            Ok(()) => println!("Task {} deleted", id),
            Err(e) => println!("Error: {}", e),
        },
    }
}