
use axum::{extract::{Path, State}, http::StatusCode, middleware, routing::get, Json, Router};
use sqlx::Row;

use crate::models::*;

pub async fn api_route() -> Router {
    let api_state = ApiState::new().await;
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
        .layer( middleware::from_fn( crate::middlewares::api_access_logger ) )
        .with_state( api_state )
}

async fn get_todos( State(api_state): State<ApiState> ) -> Json<Vec<Task>> {
    let tasks: Vec<sqlx::postgres::PgRow> = sqlx::query("SELECT * FROM tasks")
        .fetch_all(&api_state.database_connection_pool)
        .await
        .unwrap();

    let mut tasks_vec = Vec::new();
    for t in tasks.iter() {
        tasks_vec.push(
            Task {
                id: t.get::<i32, _>("id") as usize,
                title: t.get::<String, _>("title"),
                content: t.get::<String, _>("content"),
                completed: t.get::<bool, _>("completed"),
            }
        )
    }
    Json( tasks_vec )
}

async fn get_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>) -> Result<Json<Task>, StatusCode> {
    let query = sqlx::query("SELECT * FROM tasks WHERE id = $1")
        .bind(param_id as i32)
        .fetch_one(&api_state.database_connection_pool)
        .await;

    match query {
        Ok(row) => {
            Ok( Json( Task {
                id: row.get::<i32, _>("id") as usize,
                title: row.get::<String, _>("title"),
                content: row.get::<String, _>("content"),
                completed: row.get::<bool, _>("completed"),
            } ) )
        }
        Err(_) => {
            Err( StatusCode::NOT_FOUND )
        }
    }
    
}

async fn post_todo( State(api_state): State<ApiState>, Json(task): Json<TaskPost>) -> Json<Task> {
    
    let row = sqlx::query(
        r#"
            INSERT INTO tasks (title, content, completed)
            VALUES ($1, $2, $3)
            RETURNING id
        "#
    )
    .bind(task.title.clone())
    .bind(task.content.clone())
    .bind(false)
    .fetch_one(&api_state.database_connection_pool)
    .await
    .unwrap();

    let new_task = Task {
        id: row.get::<i32, _>("id") as usize,
        title: task.title,
        content: task.content,
        completed: false,
    };
    Json( new_task )
}

async fn put_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>, Json(task): Json<TaskUpdate> ) -> Result<Json<Task>, StatusCode> {
    /*
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
    */

    if task.title.is_none() || task.content.is_none() || task.completed.is_none() {
        return Err( StatusCode::BAD_REQUEST );
    }

    let query = sqlx::query("UPDATE tasks SET title = $1, content = $2, completed = $3 WHERE id = $4")
        .bind(task.title.unwrap().clone())
        .bind(task.content.unwrap().clone())
        .bind(task.completed.unwrap())
        .bind(param_id as i32)
        .execute(&api_state.database_connection_pool)
        .await;

    match query {
        Ok(_) => {
            let updated_task = sqlx::query("SELECT * FROM tasks WHERE id = $1")
                .bind(param_id as i32)
                .fetch_one(&api_state.database_connection_pool)
                .await
                .unwrap();

            Ok( 
                Json( 
                    Task {
                        id: updated_task.get::<i32, _>("id") as usize,
                        title: updated_task.get::<String, _>("title"),
                        content: updated_task.get::<String, _>("content"),
                        completed: updated_task.get::<bool, _>("completed"),
                    }
                )
            )
        }

        Err(_) => {
            Err( StatusCode::NOT_FOUND )
        }
    }
}

async fn patch_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>, Json(task): Json<TaskUpdate>) -> Result<Json<Task>, StatusCode> {

    let query = sqlx::query("SELECT * FROM tasks WHERE id = $1")
        .bind(param_id as i32)
        .fetch_one(&api_state.database_connection_pool)
        .await;

    match query {
        Ok(row) => {
            let mut title = row.get::<String, _>("title");
            let mut content = row.get::<String, _>("content");
            let mut completed = row.get::<bool, _>("completed");

            if task.title.is_some() {
                title = task.title.unwrap();
            }
            if task.content.is_some() {
                content = task.content.unwrap();
            }
            if task.completed.is_some() {
                completed = task.completed.unwrap();
            }

            let query = sqlx::query("UPDATE tasks SET title = $1, content = $2, completed = $3 WHERE id = $4")
                .bind(title.clone())
                .bind(content.clone())
                .bind(completed)
                .bind(param_id as i32)
                .execute(&api_state.database_connection_pool)
                .await;

            match query {
                Ok(_) => {
                    Ok( 
                        Json( 
                            Task {
                                id: param_id,
                                title,
                                content,
                                completed,
                            }
                        ) 
                    )
                }
                Err(_) => {
                    Err( StatusCode::NOT_FOUND )
                }
            }
        }
        Err(_) => {
            Err( StatusCode::NOT_FOUND )
        }
    }
}

async fn delete_todo_by_id( State(api_state): State<ApiState>, Path(param_id): Path<usize>) -> Result<Json<Task>, StatusCode> {
    let query = sqlx::query("SELECT * FROM tasks WHERE id = $1")
        .bind(param_id as i32)
        .fetch_one(&api_state.database_connection_pool)
        .await;

    match query {
        Ok(row) => {
            let deleted_task = Task {
                id: row.get::<i32, _>("id") as usize,
                title: row.get::<String, _>("title"),
                content: row.get::<String, _>("content"),
                completed: row.get::<bool, _>("completed"),
            };

            let delete_query = sqlx::query("DELETE FROM tasks WHERE id = $1")
                .bind(param_id as i32)
                .execute(&api_state.database_connection_pool)
                .await;

            match delete_query {
                Ok(_) => {
                    Ok( Json( deleted_task ) )
                }
                Err(_) => {
                    Err( StatusCode::NOT_FOUND )
                }
            }
        }
        Err(_) => {
            Err( StatusCode::NOT_FOUND )
        }
    }
}   
