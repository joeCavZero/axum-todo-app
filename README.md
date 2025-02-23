# Todo List

This is a simple task manager project, developed in Rust using Axum and other auxiliary libraries. The project allows adding, fetching, updating, and deleting tasks.

<div align="center"><img src="readme_assets/00.png" width="400%"></div>

## Features

- Add a new task
- List all tasks
- Fetch a task by ID
- Update an existing task
- Delete a task

<div align="center"><img src="readme_assets/01.png" width="400%"></div>

## Technologies Used

- **Rust**: Programming language used to develop the project.
- **Axum**: Web framework used to build the API.
- **SQLx**: Library to interact with the PostgreSQL database.
- **Tera**: Template engine used to render HTML pages.
- **Tokio**: Asynchronous runtime used to run the server.

<div align="center"><img src="readme_assets/02.png" width="400%"></div>

## Setup

1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/todo-list.git
    cd todo-list
    ```

2. Configure the PostgreSQL database in the [.env](http://_vscodecontentref_/3) file:
    ```properties
    DATABASE_URL=postgresql://user:password@localhost:5432/database_name
    ```

3. Install dependencies and compile the project:
    ```sh
    cargo build
    ```

4. Run the server:
    ```sh
    cargo run
    ```

<div align="center"><img src="readme_assets/03.png" width="400%"></div>

## Endpoints

- `GET /todos`: List all tasks.
- `POST /todos`: Add a new task.
- `GET /todos/{id}`: Fetch a task by ID.
- `PUT /todos/{id}`: Update a task by ID.
- `PATCH /todos/{id}`: Partially update a task by ID.
- `DELETE /todos/{id}`: Delete a task by ID.
