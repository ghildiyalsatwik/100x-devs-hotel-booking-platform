use serde::{Serialize, Deserialize};


#[derive(Deserialize)]
pub struct CreateReviewRequest {
    pub bookingId: String,
    pub rating: i32,
    pub comment: Option<String>,
}


#[derive(Serialize)]
pub struct ReviewResponse {
    pub id: String,
    pub userId: String,
    pub hotelId: String,
    pub bookingId: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub createdAt: String,
}