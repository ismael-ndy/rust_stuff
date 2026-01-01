#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn get_todo_information() -> (String, String) {
    let name = read_input("name: ");
    let description = read_input("description: ");

    (name, description)
}

pub enum Command {
    Add { name: String, description: String },
    List,
    Nothing,
    Quit,
}
pub struct CLIView;

impl CLIView {
    pub fn main_menu() {
        println!("\n=== TODO CLI ===");
        println!("[1] - Add Todo");
        println!("[2] - See All Todos");
        println!("[0] - Quit App");
    }

    pub fn read_command() -> Command {
        let input: String = read_input("> ");

        match input.as_str() {
            "1" => {
                let (name, description) = get_todo_information();
                Command::Add { name, description }
            }
            "2" => Command::List,
            "0" => Command::Quit,
            _ => {
                println!("Invalid Choice!");
                Command::Nothing
            }
        }
    }
}
