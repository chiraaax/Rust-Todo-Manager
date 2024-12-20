use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use colored::*;

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    priority: String,
    category: String,
}

fn main() {
    let file_path = "tasks.json";
    let mut tasks = load_tasks(file_path).unwrap_or_else(|_| Vec::new());

    loop {
        println!("\n{}", "To-Do List".bold().blue());
        display_tasks(&tasks);

        println!("{}", "Options:".bold().yellow());
        println!(
            "1. Add Task\n2. Remove Task\n3. Mark Completed\n4. Edit Task\n5. Sort Tasks\n6. Exit"
        );

        let choice = read_input("Enter your choice: ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => remove_task(&mut tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => edit_task(&mut tasks),
            "5" => sort_tasks(&mut tasks),
            "6" => {
                println!("{}", "Saving tasks and exiting...".bold().green());
                save_tasks(&tasks, file_path).expect("Failed to save tasks.");
                break;
            }
            _ => println!("{}", "Invalid option. Please try again.".red()),
        }
    }
}

fn load_tasks(file_path: &str) -> io::Result<Vec<Task>> {
    if Path::new(file_path).exists() {
        let file = File::open(file_path)?;
        serde_json::from_reader(file).map_err(|e| {
            eprintln!("{}", format!("Error parsing tasks: {}", e).red());
            io::Error::new(io::ErrorKind::InvalidData, e)
        })
    } else {
        Ok(Vec::new())
    }
}

fn save_tasks(tasks: &[Task], file_path: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    serde_json::to_writer(file, tasks).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn display_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("{}", "No tasks available.".bold().red());
    } else {
        println!(
            "{}",
            format!("{:<4} {:<3} {:<20} {:<10} {:<10}", "No.", "✔", "Description", "Priority", "Category")
                .bold()
                .cyan()
        );
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed {
                "[x]".bold().green()
            } else {
                "[ ]".bold().red()
            };
            let priority_color = match task.priority.to_lowercase().as_str() {
                "high" => task.priority.bold().red(),
                "medium" => task.priority.bold().yellow(),
                "low" => task.priority.bold().green(),
                _ => task.priority.normal(),
            };
            println!(
                "{:<4} {:<3} {:<20} {:<10} {:<10}",
                index + 1,
                status,
                task.description,
                priority_color,
                task.category.bold().blue()
            );
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>) {
    let description = read_input("Enter task description: ");
    let priority = read_input("Enter task priority (High, Medium, Low): ");
    let category = read_input("Enter task category (e.g., Work, Personal): ");

    tasks.push(Task {
        description,
        completed: false,
        priority,
        category,
    });
    println!("{}", "Task added successfully!".bold().green());
}

fn remove_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("{}", "No tasks to remove.".bold().red());
        return;
    }
    display_tasks(tasks);
    let index = read_input("Enter the task number to remove: ")
        .parse::<usize>()
        .ok();
    if let Some(i) = index {
        if i > 0 && i <= tasks.len() {
            tasks.remove(i - 1);
            println!("{}", "Task removed successfully!".bold().green());
        } else {
            println!("{}", "Invalid task number.".red());
        }
    } else {
        println!("{}", "Invalid input.".red());
    }
}



