// src/task.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: Option<Priority>,
    pub status: Status,
}

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String, priority: Option<Priority>) -> u32 {
        let task = Task {
            id: self.next_id,
            description,
            priority,
            status: Status::Pending,
        };
        self.tasks.push(task);
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = Status::Completed;
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    // Save tasks to a JSON file
    pub fn save_to_file(&self) -> Result<(), String> {
        let json = serde_json::to_string(&self.tasks)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?;
        std::fs::write("tasks.json", json)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }

    // Load tasks from a JSON file
    pub fn load_from_file() -> Result<Self, String> {
        let json = std::fs::read_to_string("tasks.json")
            .map_err(|_| "No tasks file found, starting fresh".to_string())?;
        let tasks: Vec<Task> = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize tasks: {}", e))?;
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(TaskManager { tasks, next_id })
    }
}