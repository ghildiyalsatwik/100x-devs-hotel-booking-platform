use axum::{extract::State, http::StatusCode, Json};
use chrono::{Utc};
use sqlx::{PgPool};
use uuid::Uuid;
use sqlx::types::BigDecimal;

use crate::{
    handlers::auth_middleware::AuthUser,
    models::{
        reviews::{CreateReviewRequest, ReviewResponse},
        response::ApiResponse,
    },
};

pub async fn create_review(
    auth: AuthUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateReviewRequest>,
) -> (StatusCode, Json<ApiResponse<ReviewResponse>>) {

    if auth.role != "customer" {
        return forbidden();
    }

    if payload.rating < 1 || payload.rating > 5 {
        return invalid_request();
    }

    let booking_id = match Uuid::parse_str(&payload.bookingId) {
        Ok(v) => v,
        Err(_) => return invalid_request(),
    };

    let mut tx = pool.begin().await.unwrap();

    let booking = sqlx::query!(
        r#"
        SELECT
            b.id,
            b.user_id,
            b.hotel_id,
            b.status,
            b.check_out_date
        FROM bookings b
        WHERE b.id = $1
        FOR UPDATE
        "#,
        booking_id
    )
    .fetch_optional(&mut *tx)
    .await
    .unwrap();

    let booking = match booking {
        Some(b) => b,
        None => {
            tx.rollback().await.unwrap();
            return booking_not_found();
        }
    };

    
    if booking.user_id != auth.user_id {
        tx.rollback().await.unwrap();
        return forbidden();
    }

    
    if booking.status != Some("confirmed".to_string())
        || booking.check_out_date >= Utc::now().date_naive()
    {
        tx.rollback().await.unwrap();
        return booking_not_eligible();
    }

    
    let exists = sqlx::query!(
        r#"SELECT id FROM reviews WHERE booking_id = $1"#,
        booking_id
    )
    .fetch_optional(&mut *tx)
    .await
    .unwrap();

    if exists.is_some() {
        tx.rollback().await.unwrap();
        return already_reviewed();
    }

    
    let review_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO reviews (
            id,
            booking_id,
            user_id,
            hotel_id,
            rating,
            comment
        )
        VALUES ($1,$2,$3,$4,$5,$6)
        "#,
        review_id,
        booking_id,
        auth.user_id,
        booking.hotel_id,
        payload.rating,
        payload.comment
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    
    let hotel = sqlx::query!(
        r#"
        SELECT rating, total_reviews
        FROM hotels
        WHERE id = $1
        FOR UPDATE
        "#,
        booking.hotel_id
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    let old_rating = hotel.rating.unwrap_or(BigDecimal::from(0));
    let total_reviews = hotel.total_reviews.unwrap_or(0);

    let new_rating = (
        (old_rating * BigDecimal::from(total_reviews))
        + BigDecimal::from(payload.rating)
    ) / BigDecimal::from(total_reviews + 1);

    sqlx::query!(
        r#"
        UPDATE hotels
        SET rating = $1,
            total_reviews = total_reviews + 1
        WHERE id = $2
        "#,
        new_rating,
        booking.hotel_id
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    let response = ReviewResponse {
        id: review_id.to_string(),
        userId: auth.user_id.to_string(),
        hotelId: booking.hotel_id.to_string(),
        bookingId: booking_id.to_string(),
        rating: payload.rating,
        comment: payload.comment,
        createdAt: Utc::now().to_rfc3339(),
    };

    (
        StatusCode::CREATED,
        Json(ApiResponse::success(response)),
    )
}

fn invalid_request() -> (StatusCode, Json<ApiResponse<ReviewResponse>>) {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::error("INVALID_REQUEST")))
}

fn forbidden() -> (StatusCode, Json<ApiResponse<ReviewResponse>>) {
    (StatusCode::FORBIDDEN, Json(ApiResponse::error("FORBIDDEN")))
}

fn booking_not_found() -> (StatusCode, Json<ApiResponse<ReviewResponse>>) {
    (StatusCode::NOT_FOUND, Json(ApiResponse::error("BOOKING_NOT_FOUND")))
}

fn already_reviewed() -> (StatusCode, Json<ApiResponse<ReviewResponse>>) {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::error("ALREADY_REVIEWED")))
}

fn booking_not_eligible() -> (StatusCode, Json<ApiResponse<ReviewResponse>>) {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::error("BOOKING_NOT_ELIGIBLE")))
}