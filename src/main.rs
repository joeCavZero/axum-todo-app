use todo_list::server;

#[ tokio::main ]
async fn main() {
    server::init("0.0.0.0", 8000).await.unwrap();
}
