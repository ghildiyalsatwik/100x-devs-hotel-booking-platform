use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::bookings::{create_booking, list_bookings};

pub fn booking_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/bookings", post(create_booking).get(list_bookings))
        .with_state(pool)
}