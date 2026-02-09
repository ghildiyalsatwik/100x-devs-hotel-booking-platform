use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub roomNumber: Option<String>,
    pub roomType: Option<String>,
    pub pricePerNight: Option<String>,
    pub maxOccupancy: Option<i32>,
}

#[derive(Serialize)]
pub struct RoomResponse {
    pub id: String,
    pub hotelId: String,
    pub roomNumber: String,
    pub roomType: String,
    pub pricePerNight: String,
    pub maxOccupancy: i32,
}