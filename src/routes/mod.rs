use axum::Router;
use sqlx::PgPool;

pub mod auth;
pub mod hotels;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(auth::auth_routes(pool.clone()))
        .merge(hotels::hotel_routes(pool))
}