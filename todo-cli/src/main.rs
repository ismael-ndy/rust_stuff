mod controller;
mod model;
mod view;

fn main() {
    let mut app = controller::TodoController::new();
    app.run();
}
