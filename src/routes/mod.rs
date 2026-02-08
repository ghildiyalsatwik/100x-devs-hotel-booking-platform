use axum::Router;
use sqlx::PgPool;

pub mod auth;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(auth::auth_routes(pool))
}