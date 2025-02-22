use crate::routes;

pub async fn init(address: &str, port: u16) -> Result<(), String > {

    let main_router = routes::main_route();
    match tokio::net::TcpListener::bind(format!("{}:{}", address, port)).await {
        Ok(listener) => {
            match axum::serve(listener, main_router).await {
                Ok(_) => {
                    Ok(())
                }
                Err(_) => {
                    Err("Axum serving failed".to_string())
                }
            }
        }
        Err(_) => {
            Err("TcpListener failed".to_string())
        }
    }
}