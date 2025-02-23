use axum::{extract::State, response::{Html, IntoResponse}, routing::get, Router};

#[derive(Clone)]
#[allow(dead_code)]
struct WebState {
}

pub fn web_router() -> Router {
    let web_state = WebState {};
    Router::new()
        .route("/", get(home))
        .route("/home", get(home))
        .route("/about", get(about))
        .route("/tasks", get(all_tasks))
        .route("/add", get(add_task))
        .route("/search", get(search_task))
        .route("/update", get(update_task))
        .route("/delete", get(delete_task))
        .with_state( web_state )
}

async fn get_html_into_response(file_path: &str, context: &tera::Context) -> impl IntoResponse {
    let file_content = std::fs::read_to_string(file_path).unwrap();
    let mut new_tera = tera::Tera::new("templates/**/*").unwrap();
    Html( new_tera.render_str(file_content.as_str(), context).unwrap() ).into_response()
}

async fn home(State(_web_state): State<WebState>) -> impl IntoResponse {
    let context = tera::Context::new();
    get_html_into_response("templates/home.html", &context).await
}

async fn about(State(_web_state): State<WebState>) -> impl IntoResponse {
    let context = tera::Context::new();
    get_html_into_response("templates/about.html", &context).await
}

async fn all_tasks(State(_web_state): State<WebState>) -> impl IntoResponse {
    let context = tera::Context::new();
    get_html_into_response("templates/all_tasks.html", &context).await
}

async fn add_task(State(_web_state): State<WebState>) -> impl IntoResponse {
    let context = tera::Context::new();
    get_html_into_response("templates/add_task.html", &context).await
}

async fn search_task(State(_web_state): State<WebState>) -> impl IntoResponse {
    let context = tera::Context::new();
    get_html_into_response("templates/search_task.html", &context).await
}

async fn update_task(State(_web_state): State<WebState>) -> impl IntoResponse {
    let context = tera::Context::new();
    get_html_into_response("templates/update_task.html", &context).await
}

async fn delete_task(State(_web_state): State<WebState>) -> impl IntoResponse {
    let context = tera::Context::new();
    get_html_into_response("templates/delete_task.html", &context).await
}