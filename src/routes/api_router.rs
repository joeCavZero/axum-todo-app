
use axum::{extract::{Path, State}, http::StatusCode, routing::get, Json, Router};

use crate::models::*;

pub fn api_route() -> Router {
    let api_state = ApiState::new();
    Router::new()
        .route(
            "/todos",        
            get(get_todos)
            .post(post_todo)
        )
        .route(
            "/todos/{id}",   
            get(get_todo_by_id)
            .put(put_todo_by_id)
            .patch(patch_todo_by_id)
            .delete(delete_todo_by_id)
        )
        .with_state( api_state )
}

async fn get_todos( State(api_state): State<ApiState> ) -> Json<Vec<Task>> {
    Json( api_state.tasks.lock().await.clone() )
}

async fn get_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>) -> Result<Json<Task>, StatusCode> {
    
    for task in api_state.tasks.lock().await.iter() {
        if task.id == param_id  {
            return Ok( Json( task.clone() ) );
        }
    }
    Err( StatusCode::NOT_FOUND )
    
}

async fn post_todo( State(api_state): State<ApiState>, Json(task): Json<TaskPost>) -> Json<Task> {
    let new_task = Task {
        id: *api_state.id_counter.lock().await,
        title: task.title,
        content: task.content,
        completed: false,
    };
    api_state.tasks.lock().await.push( new_task.clone() );
    *api_state.id_counter.lock().await += 1;
    Json( new_task )
}

async fn put_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>, Json(task): Json<TaskUpdate> ) -> Result<Json<Task>, StatusCode> {
    if task.title.is_none() || task.content.is_none() || task.completed.is_none() {
        return Err( StatusCode::BAD_REQUEST);
    }
    
    for todo_task in api_state.tasks.lock().await.iter_mut() {
        if todo_task.id == param_id {
            todo_task.title = task.title.unwrap();
            todo_task.content = task.content.unwrap();
            todo_task.completed = task.completed.unwrap();

            return Ok( Json( todo_task.clone() ) );
        }
    }

    Err( StatusCode::NOT_FOUND )
}

async fn patch_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>, Json(task): Json<TaskUpdate>) -> Result<Json<Task>, StatusCode> {
    for todo_task in api_state.tasks.lock().await.iter_mut() {
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

async fn delete_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>) -> Result<Json<Task>, StatusCode> {
    /*
     * Antes eu fiz 2 lock().await, mas estava causando um deadlock.
     * Por isso, eu fiz um lock().await e armazenei o valor em uma variável.
     * Agora funciona sem problemas.
     * 
     * Um deadlock ocorre quando um processo espera por um recurso que está sendo usado por outro processo,
     * que por sua vez está esperando pelo primeiro processo.
     * No caso, o segundo lock() estava esperando o primeiro lock() ser liberado, mas o primeiro lock() não seria liberado,
     * pois o primeiro lock() fazia parte do loop for, que obviamente não seria finalizado ali.
     */
    
    let mut tasks = api_state.tasks.lock().await;
    for (task_index, todo_task) in tasks.iter_mut().enumerate() {
        if todo_task.id == param_id {
            let task_deleted = tasks.remove(task_index);
            return Ok( Json( task_deleted ) );
        }
    }

    Err( StatusCode::NOT_FOUND )
}   
