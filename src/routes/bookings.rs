use axum::{Router, routing::{post, put}};
use sqlx::PgPool;

use crate::handlers::bookings::{create_booking, list_bookings, cancel_booking};

pub fn booking_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/bookings", post(create_booking).get(list_bookings))
        .route("/api/bookings/:bookingId/cancel", put(cancel_booking))
        .with_state(pool)
}