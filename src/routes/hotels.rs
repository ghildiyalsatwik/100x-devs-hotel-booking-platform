use axum::{Router, routing::{post, get}};
use sqlx::PgPool;

use crate::handlers::hotels::{create_hotel, list_hotels, get_hotel_by_id};

pub fn hotel_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/hotels", post(create_hotel).get(list_hotels))
        .route("/api/hotels/:hotelId", get(get_hotel_by_id))
        .with_state(pool)
}