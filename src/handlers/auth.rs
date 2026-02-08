use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;


use crate::models::{
    auth::{
        SignupRequest,
        SignupResponse,
        LoginRequest,
        LoginResponse,
        LoginUser,
    },
    response::ApiResponse,
};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

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

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    let email = match payload.email {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_login_request(),
    };

    let password = match payload.password {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_login_request(),
    };

    let user = sqlx::query!(
        r#"
        SELECT id, name, email, password_hash, role
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    let user = match user {
        Some(u) => u,
        None => return invalid_credentials(),
    };

    let valid = verify(password, &user.password_hash).unwrap_or(false);
    if !valid {
        return invalid_credentials();
    }

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET missing");
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role.clone(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    let response = LoginResponse {
        token,
        user: LoginUser {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            role: user.role,
        },
    };

    (
        StatusCode::OK,
        Json(ApiResponse::success(response)),
    )
}

fn invalid_login_request() -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::error("INVALID_REQUEST")),
    )
}

fn invalid_credentials() -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiResponse::error("INVALID_CREDENTIALS")),
    )
}