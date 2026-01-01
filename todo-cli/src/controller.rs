use crate::model::{Todo, Todos};
use crate::view::{CLIView, Command};

pub struct TodoController {
    model: Todos,
    view: CLIView,
}

impl TodoController {
    pub fn new() -> Self {
        Self {
            model: Todos::new(),
            view: CLIView::new(),
        }
    }

    fn add_todo(&mut self, name: String, description: String) {
        let todo = Todo::new(name, description);
        self.model.add_todo(todo);
    }

    fn list_todos(&self) {
        self.view.render_todos(self.model.get_todos());
    }

    fn delete_todo(&mut self) {
        let idx_res = self.view.select_todo(self.model.get_todos());

        match idx_res {
            Ok(n) => {
                self.model.remove_todo(n);
            }
            Err(e) => println!("{}", e),
        }
    }

    fn change_status(&mut self) {
        let todo_idx_res = self.view.select_todo(self.model.get_todos());

        match todo_idx_res {
            Ok(todo_idx) => {
                let status_res = self.view.select_status();
                match status_res {
                    Ok(new_status) => {
                        self.model
                            .get_todo(todo_idx)
                            .unwrap()
                            .change_status(new_status);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.view.main_menu();
            let command = self.view.read_command();

            match command {
                Some(Command::Add { name, description }) => self.add_todo(name, description),
                Some(Command::List) => self.list_todos(),
                Some(Command::ChangeStatus) => self.change_status(),
                Some(Command::Delete) => self.delete_todo(),
                Some(Command::Quit) => {
                    println!("\nGoodbye!");
                    break;
                }
                None => (),
            }
        }
    }
}
