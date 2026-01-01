# Rust Todo CLI

The first of the multiple mini projects intended to learn rust. It is a simple and clean command-line Todo app using a MVC architechture.

---

## Features

- Add todos with a title and description
- List existing todos
- Update todo status (In Progress / Abandoned / Finished)
- Delete todos

---

## Project Structure

```
src/
├── main.rs
├── controller.rs
├── view.rs
├── model.rs
```

`model.rs`

Contains the core business logic and domain types:

- Todo
- Status
- Todos (collection wrapper)

`view.rs`

Responsible for:

- Displaying menus
- Reading user input
- Formatting output

`controller.rs`

Acts as the glue:

- Receives user intent from the view
- Applies business logic via the model
- Decides what to render next

`main.rs`

- Entry point of the app

---

## Running the app

```
cargo run
```

---

## Future Work

Might add JSON persistence if I feel like it.
