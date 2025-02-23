#[derive(Clone)]
pub struct ApiState {
    pub database_connection_pool: sqlx::postgres::PgPool,
}
impl ApiState {
    pub async fn new() -> Self {
        let url = dotenvy::var("DATABASE_URL").unwrap();
        let conn_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(url.as_str())
            .await.expect("Failed to connect to Postgres");
        let _ = sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS tasks (
                    id SERIAL PRIMARY KEY,
                    title VARCHAR(100) NOT NULL,
                    content TEXT NOT NULL,
                    completed BOOLEAN NOT NULL
                )
            "#
        ).execute(&conn_pool).await.unwrap();
        


        ApiState {
            database_connection_pool: conn_pool,
        }
    }
}