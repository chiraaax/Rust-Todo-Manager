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
