#![allow(dead_code)]
#![allow(unused_variables)]

mod controller;
mod model;
mod view;

fn main() {
    let mut app = controller::TodoController::new();
    app.run();
}
