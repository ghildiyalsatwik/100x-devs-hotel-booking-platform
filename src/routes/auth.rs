use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::auth::{signup, login};

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/auth/signup", post(signup))
        .route("/api/auth/login", post(login))
        .with_state(pool)
}