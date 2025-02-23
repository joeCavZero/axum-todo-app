use axum::{body::Body, http::Request, middleware::Next, response::Response};

pub async fn web_access_logger( req: Request<Body>, next: Next ) -> Response {
    println!("[ TIMESTAMP :: {} ] [ WEB ACCESSED ] [ {} METHOD ] {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), req.method(), req.uri().path() );
    next.run(req).await
}

pub async fn api_access_logger( req: Request<Body>, next: Next ) -> Response {
    println!("[ TIMESTAMP :: {} ] [ API ACCESSED ] [ {} METHOD ] {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), req.method(), req.uri().path() );
    next.run(req).await
}