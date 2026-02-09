use axum::{extract::{State, Query}, http::StatusCode, Json};
use bigdecimal::ToPrimitive;
use sqlx::PgPool;
use uuid::Uuid;
use std::str::FromStr;
use sqlx::types::BigDecimal;

use crate::{
    handlers::auth_middleware::AuthUser,
    models::{
        hotels::{CreateHotelRequest, HotelResponse, HotelSearchQuery, HotelListResponse},
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

pub async fn list_hotels(
    _auth: AuthUser,
    State(pool): State<PgPool>,
    Query(filters): Query<HotelSearchQuery>,
) -> (StatusCode, Json<ApiResponse<Vec<HotelListResponse>>>) {

    let min_price: Option<BigDecimal> = filters
    .minPrice
    .as_deref()
    .and_then(|v| BigDecimal::from_str(v).ok());

    let max_price: Option<BigDecimal> = filters
    .maxPrice
    .as_deref()
    .and_then(|v| BigDecimal::from_str(v).ok());

    let hotels = sqlx::query!(
        r#"
        SELECT
            h.id,
            h.name,
            h.description,
            h.city,
            h.country,
            h.amenities,
            h.rating,
            h.total_reviews,
            MIN(r.price_per_night) AS min_price
        FROM hotels h
        JOIN rooms r ON r.hotel_id = h.id
        WHERE
            ($1::text IS NULL OR LOWER(h.city) = LOWER($1))
        AND ($2::text IS NULL OR LOWER(h.country) = LOWER($2))
        AND ($3::numeric IS NULL OR r.price_per_night >= $3)
        AND ($4::numeric IS NULL OR r.price_per_night <= $4)
        AND ($5::float8 IS NULL OR h.rating >= $5)
        GROUP BY h.id
        "#,
        filters.city,
        filters.country,
        min_price,
        max_price,
        filters.minRating,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let response = hotels
    .into_iter()
    .map(|h| HotelListResponse {
        id: h.id.to_string(),
        name: h.name,
        description: h.description,
        city: h.city,
        country: h.country,
        amenities: h.amenities.unwrap_or_default(),
        rating: h.rating.and_then(|r| r.to_f64()).unwrap_or(0.0),
        totalReviews: h.total_reviews.unwrap_or(0),
        minPricePerNight: h
            .min_price
            .map(|v| v.to_string())
            .unwrap_or_else(|| "0".to_string()),
    })
    .collect();

    (
        StatusCode::OK,
        Json(ApiResponse::success(response)),
    )
}