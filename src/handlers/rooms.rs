use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use sqlx::types::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    handlers::auth_middleware::AuthUser,
    models::{
        rooms::{CreateRoomRequest, RoomResponse},
        response::ApiResponse,
    },
};

pub async fn create_room(
    AuthUser { user_id, role }: AuthUser,
    State(pool): State<PgPool>,
    Path(hotel_id): Path<Uuid>,
    Json(payload): Json<CreateRoomRequest>,
) -> (StatusCode, Json<ApiResponse<RoomResponse>>) {
    
    if role != "owner" {
        return forbidden();
    }

    
    let room_number = match payload.roomNumber {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    
    let room_type = match payload.roomType {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    
    let price_str = match payload.pricePerNight {
        Some(v) if !v.trim().is_empty() => v,
        _ => return invalid_request(),
    };

    let price = match BigDecimal::from_str(&price_str) {
        Ok(v) if v > BigDecimal::from(0) => v,
        _ => return invalid_request(),
    };

    
    let occupancy = match payload.maxOccupancy {
        Some(v) if v > 0 => v,
        _ => return invalid_request(),
    };

    
    let hotel = sqlx::query!(
        r#"
        SELECT owner_id
        FROM hotels
        WHERE id = $1
        "#,
        hotel_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    let hotel = match hotel {
        Some(h) => h,
        None => return not_found(),
    };

    if hotel.owner_id != user_id {
        return forbidden();
    }

    
    let exists = sqlx::query!(
        r#"
        SELECT id
        FROM rooms
        WHERE hotel_id = $1 AND room_number = $2
        "#,
        hotel_id,
        room_number
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if exists.is_some() {
        return room_exists();
    }

    
    let room_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO rooms (
            id,
            hotel_id,
            room_number,
            room_type,
            price_per_night,
            max_occupancy
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        room_id,
        hotel_id,
        room_number,
        room_type,
        price,
        occupancy
    )
    .execute(&pool)
    .await
    .unwrap();

    
    let response = RoomResponse {
        id: room_id.to_string(),
        hotelId: hotel_id.to_string(),
        roomNumber: room_number,
        roomType: room_type,
        pricePerNight: price.to_string(),
        maxOccupancy: occupancy,
    };

    (
        StatusCode::CREATED,
        Json(ApiResponse::success(response)),
    )
}


fn invalid_request() -> (StatusCode, Json<ApiResponse<RoomResponse>>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::error("INVALID_REQUEST")),
    )
}

fn forbidden() -> (StatusCode, Json<ApiResponse<RoomResponse>>) {
    (
        StatusCode::FORBIDDEN,
        Json(ApiResponse::error("FORBIDDEN")),
    )
}

fn not_found() -> (StatusCode, Json<ApiResponse<RoomResponse>>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse::error("HOTEL_NOT_FOUND")),
    )
}

fn room_exists() -> (StatusCode, Json<ApiResponse<RoomResponse>>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::error("ROOM_ALREADY_EXISTS")),
    )
}