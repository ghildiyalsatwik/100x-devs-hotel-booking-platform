use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    
    dotenvy::dotenv().ok();

    let app = Router::new();

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
