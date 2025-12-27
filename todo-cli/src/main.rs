use std::fmt;
use std::io;
use std::io::Write;

enum Status {
    InProgress,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::InProgress => write!(f, "IN PROGRESS"),
        }
    }
}

struct Todo {
    name: String,
    description: String,
    status: Status,
}

fn add_todo(todos: &mut Vec<Todo>) {
    let my_name = read_input("\nname: ");
    let my_description = read_input("description: ");

    let todo: Todo = Todo {
        name: my_name,
        description: my_description,
        status: Status::InProgress,
    };

    todos.push(todo);
    println!("Todo successfully added!");
}

fn see_todo(todos: &Vec<Todo>) {
    for (i, todo) in todos.iter().enumerate() {
        println!("\nTODO {}", i);
        println!(
            "Name: {}\nDescription: {}\nStatus: {}",
            todo.name, todo.description, todo.status
        );
    }
}

fn read_input(prompt: &str) -> String {
    let mut input: String = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_string();
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!("\n=== TODO APP ===");
        println!("1. Add todo");
        println!("2. View todos");
        println!("0. Quit");

        let choice = read_input(">> ");

        match choice.as_str() {
            "1" => add_todo(&mut todos),
            "2" => see_todo(&todos),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown choice! Try again."),
        }
    }
}
