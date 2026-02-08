use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    auth::{SignupRequest, SignupResponse},
    response::ApiResponse,
};

pub async fn signup(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> (StatusCode, Json<ApiResponse<SignupResponse>>) {

    let name = match payload.name {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    let email = match payload.email {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    let password = match payload.password {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    let role = payload.role.unwrap_or_else(|| "customer".to_string());

    if role != "customer" && role != "owner" {
        return invalid_request();
    }

    
    let existing = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        email
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if existing.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("EMAIL_ALREADY_EXISTS")),
        );
    }

    
    let password_hash = hash(password, DEFAULT_COST).unwrap();

    
    let user_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO users (id, name, email, password_hash, role, phone)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        user_id,
        name,
        email,
        password_hash,
        role,
        payload.phone
    )
    .execute(&pool)
    .await
    .unwrap();

    let response = SignupResponse {
        id: user_id.to_string(),
        name,
        email,
        role,
        phone: payload.phone,
    };

    (
        StatusCode::CREATED,
        Json(ApiResponse::success(response)),
    )
}

fn invalid_request() -> (StatusCode, Json<ApiResponse<SignupResponse>>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::error("INVALID_REQUEST")),
    )
}