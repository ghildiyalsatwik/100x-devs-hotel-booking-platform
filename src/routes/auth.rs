use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::auth::signup;

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/auth/signup", post(signup))
        .with_state(pool)
}