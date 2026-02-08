use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::models::response::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub struct AuthUser {
    pub user_id: Uuid,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<ApiResponse<()>>);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok());

        let token = match auth_header {
            Some(v) if v.starts_with("Bearer ") => &v[7..],
            _ => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    axum::Json(ApiResponse::error("UNAUTHORIZED")),
                ))
            }
        };

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET missing");

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(ApiResponse::error("UNAUTHORIZED")),
            )
        })?;

        let user_id = Uuid::parse_str(&decoded.claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(ApiResponse::error("UNAUTHORIZED")),
            )
        })?;
        
        Ok(AuthUser {
            user_id,
            role: decoded.claims.role,
        })
    }
}