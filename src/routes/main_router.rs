use axum::{middleware, Router};
use tower_http::services::ServeDir;

use super::{api_router::api_route, web_router::web_router};



pub async fn main_route() -> axum::Router {
    Router::new()
        .nest("/api", api_route().await )
        .merge( web_router() )
        .nest_service("/static", ServeDir::new("./static") )
        .layer( middleware::from_fn( crate::middlewares::normalize_trailing_slash ) )

}