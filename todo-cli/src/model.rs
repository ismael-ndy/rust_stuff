#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

#[derive(Debug)]
pub enum Status {
    InProgress,
    Abandoned,
    Finished,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Todo {
    name: String,
    description: String,
    status: Status,
}

impl Todo {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name: name,
            description: description,
            status: Status::InProgress,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn change_status(&mut self, status: Status) -> () {
        self.status = status;
    }
}

pub struct Todos {
    pub todos: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, todo: Todo) -> () {
        self.todos.push(todo);
    }

    pub fn get_todo(&mut self, idx: usize) -> Option<&mut Todo> {
        self.todos.get_mut(idx)
    }

    pub fn remove_todo(&mut self, idx: usize) -> Option<Todo> {
        if idx < self.todos.len() {
            Some(self.todos.remove(idx))
        } else {
            None
        }
    }
}
