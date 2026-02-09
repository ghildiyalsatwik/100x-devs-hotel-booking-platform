use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateBookingRequest {
    pub roomId: String,
    pub checkInDate: String,
    pub checkOutDate: String,
    pub guests: i32,
}

#[derive(Serialize)]
pub struct BookingResponse {
    pub id: String,
    pub userId: String,
    pub roomId: String,
    pub hotelId: String,
    pub checkInDate: String,
    pub checkOutDate: String,
    pub guests: i32,
    pub totalPrice: String,
    pub status: String,
    pub bookingDate: String,
}

#[derive(Deserialize)]
pub struct BookingListQuery {
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct BookingListResponse {
    pub id: String,
    pub roomId: String,
    pub hotelId: String,
    pub hotelName: String,
    pub roomNumber: String,
    pub roomType: String,
    pub checkInDate: String,
    pub checkOutDate: String,
    pub guests: i32,
    pub totalPrice: String,
    pub status: String,
    pub bookingDate: String,
}