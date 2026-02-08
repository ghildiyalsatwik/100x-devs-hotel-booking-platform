use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::hotels::create_hotel;

pub fn hotel_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/hotels", post(create_hotel))
        .with_state(pool)
}