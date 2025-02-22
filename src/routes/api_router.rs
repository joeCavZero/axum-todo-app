use axum::{extract::Path, http::StatusCode, routing::{delete, get, patch, post, put}, Json, Router};

use crate::models::*;

static mut TODOS: Vec<Task> = vec![];
static mut ID_COUNTER: usize = 0;
pub fn api_route() -> Router {
    Router::new()
        .route("/todos",        get(get_todos))
        .route("/todos/{id}",   get(get_todo_by_id))
        .route("/todos",        post(post_todo))
        .route("/todos/{id}",   put(put_todo_by_id))
        .route("/todos/{id}",   patch(patch_todo_by_id))
        .route("/todos/{id}",   delete(delete_todo_by_id))
}

async fn get_todos() -> Json<Vec<Task>> {
    unsafe{ Json(  TODOS.clone()  ) }
}

async fn get_todo_by_id(Path(param_id): Path<usize>) -> Result<Json<Task>, StatusCode> {
    unsafe {
        for task in TODOS.iter() {
            if task.id == param_id  {
                return Ok( Json( task.clone() ) );
            }
        }
        Err( StatusCode::NOT_FOUND )
    }
}

async fn post_todo(Json(task): Json<TaskPost>) -> Json<Task> {
    let new_task = Task {
        id: unsafe{ ID_COUNTER },
        title: task.title,
        content: task.content,
        completed: false,
    };
    unsafe { TODOS.push( new_task.clone() ) }
    unsafe { ID_COUNTER += 1 }
    Json( new_task )
}

async fn put_todo_by_id( Path(param_id): Path<usize>, Json(task): Json<TaskUpdate> ) -> Result<Json<Task>, StatusCode> {
    if task.title.is_none() || task.content.is_none() || task.completed.is_none() {
        return Err( StatusCode::BAD_REQUEST);
    }
    
    for todo_task in unsafe { TODOS.iter_mut() } {
        if todo_task.id == param_id {
            todo_task.title = task.title.unwrap();
            todo_task.content = task.content.unwrap();
            todo_task.completed = task.completed.unwrap();

            return Ok( Json( todo_task.clone() ) );
        }
    }

    Err( StatusCode::NOT_FOUND )
}

async fn patch_todo_by_id(Path(param_id): Path<usize>, Json(task): Json<TaskUpdate>) -> Result<Json<Task>, StatusCode> {
    for todo_task in unsafe{ TODOS.iter_mut() } {
        if todo_task.id == param_id {
            if task.title.is_some() {
                todo_task.title = task.title.unwrap();
            }
            if task.content.is_some() {
                todo_task.content = task.content.unwrap()
            }

            if task.completed.is_some() {
                todo_task.completed = task.completed.unwrap();
            }

            return Ok( Json(todo_task.clone()) );
        }
    }

    Err( StatusCode::NOT_FOUND )
}

async fn delete_todo_by_id(Path(param_id): Path<usize>) -> Result<Json<Task>, StatusCode> {
    for (task_index, todo_task) in unsafe{ TODOS.iter().enumerate() } {
        if todo_task.id == param_id {
            unsafe{ return Ok( Json(TODOS.remove(task_index) ) ) };
        }
    }

    Err( StatusCode::NOT_FOUND )
}   
