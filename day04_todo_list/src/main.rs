use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: Uuid,
    title: String,
    completed: bool,
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!(
            r"
  ┌───────────────────────┐
  |    No Tasks Found     |
  └───────────────────────┘
        "
        );
    } else {
        println!(
            r"
  ╔═══════════════════════╗
  ║       TASK LIST       ║
  ╚═══════════════════════╝
        "
        );

        for task in tasks {
            println!(
                "  [{}] {} (ID: {})",
                if task.completed { "✔" } else { " " },
                task.title.trim(),
                task.id
            );
        }

        println!(
            r"
  ═══════════════════════
        "
        );
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Please Enter the description of your task");
    let mut task_name = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to read your input please try again");
    let task = Task {
        id: Uuid::new_v4(),
        title: task_name,
        completed: false,
    };
    tasks.push(task);
    println!("Task Added Successfully");
    view_tasks(tasks);
    save_task(tasks);
}
fn mark_done(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available to mark as done.");
        return;
    }
    let mut task_id = String::new();
    println!("Enter the id of the task to be marked done");
    io::stdin()
        .read_line(&mut task_id)
        .expect("Error reading your task id");
    let trimmed_id = task_id.trim();
    for task in tasks.iter_mut() {
        if task.id.to_string() == trimmed_id {
            task.completed = true;
        }
    }
    save_task(tasks);
}
fn save_task(tasks: &mut Vec<Task>) {
    let serialized_task = serde_json::to_string(tasks).expect("Error reading the task");
    fs::write("list.clm", serialized_task).expect("Error saving the file");
    println!("Saved Successfully");
}
fn load_task() -> Vec<Task> {
    match fs::read_to_string("list.clm") {
        Ok(n) => {
            let deserialized_tasks: Vec<Task> = serde_json::from_str(&n).unwrap_or(Vec::new());
            println!("Loaded Successfully");
            return deserialized_tasks;
        }
        Err(_e) => {
            fs::write("list.clm", "").expect("Error Creating file");
            fs::read_to_string("list.clm").expect("Error creating file");
            return Vec::new();
        }
    };
}
fn main() {
    let mut tasks: Vec<Task> = load_task();
    println!(
        "
=== To-Do List ===
     1. View tasks
     2. Add task
     3. Mark task as done
     4. Exit
     Choose an option:
"
    );
    let mut entry_option = String::new();
    io::stdin()
        .read_line(&mut entry_option)
        .expect("Error reading user input please try again");

    match entry_option.trim().parse::<i32>() {
        Ok(1) => view_tasks(&tasks),
        Ok(2) => add_task(&mut tasks),
        Ok(3) => mark_done(&mut tasks),
        Ok(4) => {
            println!("Thanks for using this application!");
            return;
        }
        _ => eprintln!("Please enter a valid option."),
    }
}
