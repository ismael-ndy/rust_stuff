use std::fmt;
use std::io;
use std::io::Write;
use std::usize;

#[derive(Clone)]
enum Status {
    InProgress,
    Abandonned,
    Finished,
}

const ALL_STATUS: [Status; 3] = [Status::InProgress, Status::Abandonned, Status::Finished];

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::InProgress => write!(f, "IN PROGRESS"),
            Self::Abandonned => write!(f, "ABANDONNED"),
            Self::Finished => write!(f, "FINISHED"),
        }
    }
}

struct Todo {
    name: String,
    description: String,
    status: Status,
}

fn add_todo(todos: &mut Vec<Todo>) -> () {
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

fn see_todo(todos: &Vec<Todo>) -> () {
    for (i, todo) in todos.iter().enumerate() {
        println!("\nTODO {}", i);
        println!(
            "Name: {}\nDescription: {}\nStatus: {}",
            todo.name, todo.description, todo.status
        );
    }
}

fn parse_int_in_range(input: &str, n: &usize) -> Result<usize, String> {
    let value: usize = input.parse::<usize>().map_err(|_| "Invalid choice.")?;

    if (0..*n).contains(&value) {
        return Ok(value);
    } else {
        return Err(format!("Invalid choice"));
    }
}

fn change_todo_status(todos: &mut Vec<Todo>) {
    let n: usize = todos.len();
    let todo_to_modify: &mut Todo;

    println!("\n=== TODOS ===");
    for (i, todo) in todos.iter().enumerate() {
        println!("[{}] - {}", i, todo.name);
    }
    let choice: String = read_input("Choose a todo to modify: ");

    match parse_int_in_range(&choice, &n) {
        Ok(v) => todo_to_modify = &mut todos[v],
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("\nCurrent todo: ");
    println!(
        "Name: {}\nDescription: {}\nStatus: {}",
        todo_to_modify.name, todo_to_modify.description, todo_to_modify.status
    );
    println!("\nStatus available: ");
    for (i, status) in ALL_STATUS.iter().enumerate() {
        println!("[{}] - {}", i, status);
    }
    let choice: String = read_input("New status: ");
    match parse_int_in_range(&choice, &ALL_STATUS.len()) {
        Ok(v) => todo_to_modify.status = ALL_STATUS[v].clone(),
        Err(e) => println!("Error: {}", e),
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
        println!("3. Change todo status");
        println!("0. Quit");

        let choice = read_input(">> ");

        match choice.as_str() {
            "1" => add_todo(&mut todos),
            "2" => see_todo(&todos),
            "3" => change_todo_status(&mut todos),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown choice! Try again."),
        }
    }
}
