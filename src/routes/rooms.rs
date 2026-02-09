use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::rooms::create_room;

pub fn room_routes(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/api/hotels/:hotelId/rooms",
            post(create_room),
        )
        .with_state(pool)
}