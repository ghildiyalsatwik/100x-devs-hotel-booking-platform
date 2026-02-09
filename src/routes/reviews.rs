use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::reviews::create_review;

pub fn review_route(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/api/reviews",
            post(create_review),
        )
        .with_state(pool)
}