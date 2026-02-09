use axum::{extract::{State, Query}, http::StatusCode, Json};
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use sqlx::types::BigDecimal;

use crate::{
    handlers::auth_middleware::AuthUser,
    models::{
        bookings::{CreateBookingRequest, BookingResponse, BookingListQuery, BookingListResponse},
        response::ApiResponse,
    },
};

pub async fn create_booking(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateBookingRequest>,
) -> (StatusCode, Json<ApiResponse<BookingResponse>>) {

    
    if auth.role != "customer" {
        return forbidden();
    }

    
    let room_id = match Uuid::parse_str(&payload.roomId) {
        Ok(v) => v,
        Err(_) => return invalid_request(),
    };

    
    let check_in = NaiveDate::parse_from_str(&payload.checkInDate, "%Y-%m-%d").ok();
    let check_out = NaiveDate::parse_from_str(&payload.checkOutDate, "%Y-%m-%d").ok();

    let (check_in, check_out) = match (check_in, check_out) {
        (Some(ci), Some(co)) if ci < co && ci >= Utc::now().date_naive() => (ci, co),
        _ => return invalid_dates(),
    };

    let nights = (check_out - check_in).num_days();
    if nights <= 0 {
        return invalid_dates();
    }

    let mut tx = pool.begin().await.unwrap();

    
    let room = sqlx::query!(
        r#"
        SELECT
            r.id,
            r.hotel_id,
            r.price_per_night,
            r.max_occupancy,
            h.owner_id
        FROM rooms r
        JOIN hotels h ON h.id = r.hotel_id
        WHERE r.id = $1
        FOR UPDATE
        "#,
        room_id
    )
    .fetch_optional(&mut *tx)
    .await
    .unwrap();

    let room = match room {
        Some(r) => r,
        None => {
            tx.rollback().await.unwrap();
            return room_not_found();
        }
    };

    
    if room.owner_id == auth.user_id {
        tx.rollback().await.unwrap();
        return forbidden();
    }

    
    if payload.guests > room.max_occupancy {
        tx.rollback().await.unwrap();
        return invalid_capacity();
    }

    
    let overlap = sqlx::query!(
        r#"
        SELECT id FROM bookings
        WHERE room_id = $1
        AND status = 'confirmed'
        AND NOT (
            check_out_date <= $2
            OR check_in_date >= $3
        )
        "#,
        room_id,
        check_in,
        check_out
    )
    .fetch_optional(&mut *tx)
    .await
    .unwrap();

    if overlap.is_some() {
        tx.rollback().await.unwrap();
        return room_not_available();
    }

    
    let total_price = &room.price_per_night * BigDecimal::from(nights);

    
    let booking_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO bookings (
            id,
            user_id,
            room_id,
            hotel_id,
            check_in_date,
            check_out_date,
            guests,
            total_price,
            status
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,'confirmed')
        "#,
        booking_id,
        auth.user_id,
        room_id,
        room.hotel_id,
        check_in,
        check_out,
        payload.guests,
        total_price
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    
    let response = BookingResponse {
        id: booking_id.to_string(),
        userId: auth.user_id.to_string(),
        roomId: room_id.to_string(),
        hotelId: room.hotel_id.to_string(),
        checkInDate: payload.checkInDate,
        checkOutDate: payload.checkOutDate,
        guests: payload.guests,
        totalPrice: total_price.to_string(),
        status: "confirmed".to_string(),
        bookingDate: Utc::now().to_rfc3339(),
    };

    (
        StatusCode::CREATED,
        Json(ApiResponse::success(response)),
    )
}

fn invalid_request() -> (StatusCode, Json<ApiResponse<BookingResponse>>) {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::error("INVALID_REQUEST")))
}

fn invalid_dates() -> (StatusCode, Json<ApiResponse<BookingResponse>>) {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::error("INVALID_DATES")))
}

fn invalid_capacity() -> (StatusCode, Json<ApiResponse<BookingResponse>>) {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::error("INVALID_CAPACITY")))
}

fn room_not_available() -> (StatusCode, Json<ApiResponse<BookingResponse>>) {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::error("ROOM_NOT_AVAILABLE")))
}

fn room_not_found() -> (StatusCode, Json<ApiResponse<BookingResponse>>) {
    (StatusCode::NOT_FOUND, Json(ApiResponse::error("ROOM_NOT_FOUND")))
}

fn forbidden() -> (StatusCode, Json<ApiResponse<BookingResponse>>) {
    (StatusCode::FORBIDDEN, Json(ApiResponse::error("FORBIDDEN")))
}


pub async fn list_bookings(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Query(filters): Query<BookingListQuery>,
) -> (StatusCode, Json<ApiResponse<Vec<BookingListResponse>>>) {

    if auth.role != "customer" {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("FORBIDDEN")),
        );
    }

    let bookings = sqlx::query!(
        r#"
        SELECT
            b.id,
            b.room_id,
            b.hotel_id,
            h.name AS hotel_name,
            r.room_number,
            r.room_type,
            b.check_in_date,
            b.check_out_date,
            b.guests,
            b.total_price,
            b.status,
            b.booking_date
        FROM bookings b
        JOIN rooms r ON r.id = b.room_id
        JOIN hotels h ON h.id = b.hotel_id
        WHERE
            b.user_id = $1
        AND ($2::text IS NULL OR b.status = $2)
        ORDER BY b.booking_date DESC
        "#,
        auth.user_id,
        filters.status,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let response = bookings
        .into_iter()
        .map(|b| BookingListResponse {
            id: b.id.to_string(),
            roomId: b.room_id.to_string(),
            hotelId: b.hotel_id.to_string(),
            hotelName: b.hotel_name,
            roomNumber: b.room_number,
            roomType: b.room_type,
            checkInDate: b.check_in_date.to_string(),
            checkOutDate: b.check_out_date.to_string(),
            guests: b.guests,
            totalPrice: b.total_price.to_string(),
            status: b.status.unwrap_or_else(|| "confirmed".to_string()),
            bookingDate: b.booking_date
            .map(|d| d.and_utc().to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339()),
        })
        .collect();

    (
        StatusCode::OK,
        Json(ApiResponse::success(response)),
    )
}