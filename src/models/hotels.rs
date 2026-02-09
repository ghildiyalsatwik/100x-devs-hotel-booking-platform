use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateHotelRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub amenities: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct HotelResponse {
    pub id: String,
    pub ownerId: String,
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub country: String,
    pub amenities: Vec<String>,
    pub rating: f64,
    pub totalReviews: i32,
}

#[derive(Deserialize)]
pub struct HotelSearchQuery {
    pub city: Option<String>,
    pub country: Option<String>,
    pub minPrice: Option<String>,
    pub maxPrice: Option<String>,
    pub minRating: Option<f64>,
}

#[derive(Serialize)]
pub struct HotelListResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub country: String,
    pub amenities: Vec<String>,
    pub rating: f64,
    pub totalReviews: i32,
    pub minPricePerNight: String,
}

#[derive(Serialize)]
pub struct HotelDetailResponse {
    pub id: String,
    pub ownerId: String,
    pub name: String,
    pub description: Option<String>,
    pub city: String,
    pub country: String,
    pub amenities: Vec<String>,
    pub rating: f64,
    pub totalReviews: i32,
    pub rooms: Vec<HotelRoomResponse>,
}

#[derive(Serialize)]
pub struct HotelRoomResponse {
    pub id: String,
    pub roomNumber: String,
    pub roomType: String,
    pub pricePerNight: String,
    pub maxOccupancy: i32,
}