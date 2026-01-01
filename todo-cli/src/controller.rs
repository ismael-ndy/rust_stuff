#![allow(dead_code)]
#![allow(unused_variables)]

use crate::model::{Todo, Todos};
use crate::view::{CLIView, Command};

pub struct TodoController {
    model: Todos,
}

impl TodoController {
    pub fn new() -> Self {
        Self {
            model: Todos::new(),
        }
    }

    fn add_todo(&mut self, name: String, description: String) {
        let todo = Todo::new(name, description);
        self.model.add_todo(todo);
    }

    fn list_todos(&self) {
        for (i, todo) in self.model.todos.iter().enumerate() {
            println!("\nTodo {i}");
            println!(
                "Name: {}\nDescription: {}\nStatus: {}",
                todo.get_name(),
                todo.get_description(),
                todo.get_status()
            );
        }
    }

    pub fn run(&mut self) {
        loop {
            CLIView::main_menu();
            let command = CLIView::read_command();

            match command {
                Command::Add { name, description } => self.add_todo(name, description),
                Command::List => self.list_todos(),
                Command::Nothing => (),
                Command::Quit => {
                    println!("\nGoodbye!");
                    break;
                }
            }
        }
    }
}
