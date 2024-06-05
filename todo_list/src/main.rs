use std::io::{self, Write};

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\n1. Add task");
        println!("2. View tasks");
        println!("3. Complete task");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => add_task(&mut todos),
            "2" => view_tasks(&todos),
            "3" => complete_task(&mut todos),
            "4" => break,
            _ => println!("Invalid choice. Please choose a number between 1 and 4."),
        }
    }
}

fn add_task(todos: &mut Vec<String>) {
    let mut task = String::new();
    println!("Enter the task to add:");
    io::stdin().read_line(&mut task).expect("Failed to read line");
    todos.push(task.trim().to_string());
    println!("Task added.");
}

fn view_tasks(todos: &[String]) {
    if todos.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Todo list:");
        for (i, task) in todos.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}

fn complete_task(todos: &mut Vec<String>) {
    let mut input = String::new();
    println!("Enter the number of the task to complete:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let task_num: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number. Please enter a valid task number.");
            return;
        }
    };

    if task_num == 0 || task_num > todos.len() {
        println!("Invalid task number.");
    } else {
        todos.remove(task_num - 1);
        println!("Task completed.");
    }
}
