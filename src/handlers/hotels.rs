use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    handlers::auth_middleware::AuthUser,
    models::{
        hotels::{CreateHotelRequest, HotelResponse},
        response::ApiResponse,
    },
};

pub async fn create_hotel(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateHotelRequest>,
) -> (StatusCode, Json<ApiResponse<HotelResponse>>) {
    
    if auth.role != "owner" {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("FORBIDDEN")),
        );
    }

    
    let name = match payload.name {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    let city = match payload.city {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    let country = match payload.country {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    let amenities = payload.amenities.unwrap_or_default();

    let hotel_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO hotels (
            id, owner_id, name, description, city, country, amenities
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        hotel_id,
        auth.user_id,
        name,
        payload.description,
        city,
        country,
        &amenities
    )
    .execute(&pool)
    .await
    .unwrap();

    let response = HotelResponse {
        id: hotel_id.to_string(),
        ownerId: auth.user_id.to_string(),
        name,
        description: payload.description,
        city,
        country,
        amenities,
        rating: 0.0,
        totalReviews: 0,
    };

    (
        StatusCode::CREATED,
        Json(ApiResponse::success(response)),
    )
}

fn invalid_request() -> (StatusCode, Json<ApiResponse<HotelResponse>>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::error("INVALID_REQUEST")),
    )
}