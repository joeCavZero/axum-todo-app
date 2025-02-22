use axum::{body::Body, http::Request, middleware::Next, response::{IntoResponse, Redirect, Response}};

pub async fn normalize_trailing_slash( req: Request<Body>, next: Next ) -> Response {
    /*
     *  Every middleware must have the same signature:
     *      -> async fn middleware( req: Request<Body>, next: Next ) -> Response
     *  -> The 'req' is the request that comes from the client,
     *  -> The 'next' is the next middleware or the handler that will be called.
     *  -> The 'Response' is the response that will be sent to the client.
     */
    
    let path = req.uri().path().to_string();
    println!("entering middleware: {}", path);
    if path == "/" {
        next.run(req).await
    } else if path.ends_with('/') {
        let new_path = path[0..path.len()-1].to_string();
        Redirect::permanent(new_path.as_str()).into_response()
    } else {
        next.run(req).await
    }
    /*
        This is another way to do the normalization,
        but the .route paths must have the trailing slash
        at the end of the 'path string'.
    if path.ends_with('/') || path == "/" {
        next.run(req).await
    } else {
        let new_path = format!("{}/", path);
        Redirect::permanent(new_path.as_str()).into_response()
    }
    */
}