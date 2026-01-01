use crate::model::{Status, Todo};
use std::io::{self, Write};
use strum::{IntoEnumIterator, VariantArray};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn parse_int_in_range(input: &str, n: usize) -> Result<usize, String> {
    let value = input.parse::<usize>().map_err(|_| "Invalid Choice!")?;

    if value < n {
        Ok(value)
    } else {
        Err(format!("Invalid Choice!"))
    }
}

fn get_todo_information() -> (String, String) {
    let name = read_input("name: ");
    let description = read_input("description: ");

    (name, description)
}

pub enum Command {
    Add { name: String, description: String },
    List,
    ChangeStatus,
    Delete,
    Quit,
}

pub struct CLIView;

impl CLIView {
    pub fn new() -> Self {
        Self
    }

    pub fn main_menu(&self) {
        println!("\n=== TODO CLI ===");
        println!("[1] - Add Todo");
        println!("[2] - See All Todos");
        println!("[3] - Change Todo Status");
        println!("[4] - Delete Todo");
        println!("[0] - Quit App");
    }

    pub fn read_command(&self) -> Option<Command> {
        let input: String = read_input("> ");

        match input.as_str() {
            "1" => {
                let (name, description) = get_todo_information();
                Some(Command::Add { name, description })
            }
            "2" => Some(Command::List),
            "3" => Some(Command::ChangeStatus),
            "4" => Some(Command::Delete),
            "0" => Some(Command::Quit),
            _ => {
                println!("Invalid Choice!");
                None
            }
        }
    }

    pub fn render_todos(&self, todos: &Vec<Todo>) {
        for (i, todo) in todos.iter().enumerate() {
            println!("\nTodo {i}");
            println!(
                "Name: {}\nDescription: {}\nStatus: {}",
                todo.get_name(),
                todo.get_description(),
                todo.get_status()
            );
        }
    }

    pub fn select_todo(&self, todos: &Vec<Todo>) -> Result<usize, String> {
        self.render_todos(todos);
        let input = read_input("> ");
        let idx = parse_int_in_range(&input, todos.len());

        idx
    }

    pub fn select_status(&self) -> Result<Status, String> {
        for (i, status) in Status::iter().enumerate() {
            println!("[{i}] - {status}");
        }

        let input = read_input("> ");
        let idx = parse_int_in_range(&input, Status::VARIANTS.len());

        match idx {
            Ok(n) => Ok(Status::VARIANTS[n]),
            Err(e) => Err(e),
        }
    }
}
