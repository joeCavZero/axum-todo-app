use axum::{middleware, Router};
use tower_http::services::ServeDir;

use super::api_router::api_route;



pub fn main_route() -> axum::Router {
    Router::new()
        .nest("/api",       api_route() )
        .fallback_service( ServeDir::new("./static") )
        .layer( middleware::from_fn( crate::middlewares::normalize_trailing_slash ) )

}