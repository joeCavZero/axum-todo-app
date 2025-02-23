use crate::routes;

pub async fn init(address: &str, port: u16) -> Result<(), String > {
    dotenvy::dotenv().unwrap();

    let main_router = routes::main_route().await;
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