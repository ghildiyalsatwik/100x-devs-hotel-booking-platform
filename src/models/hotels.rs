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