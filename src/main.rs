use tokio::net::TcpListener;
use std::env;

mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    
    dotenvy::dotenv().ok();

    let addr = env::var("ADDRESS").expect("Server address not set in .env file!");

    let pool = db::create_pool().await;

    let app = routes::create_routes(pool);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap();
    
    println!("Server running!");

    axum::serve(listener, app)
        .await
        .unwrap();
}