use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::hotels::{create_hotel, list_hotels};

pub fn hotel_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/hotels", post(create_hotel).get(list_hotels))
        .with_state(pool)
}