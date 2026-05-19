use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventInventoryResponse {
    pub event_id: u64,
    pub event: String,
    pub capacity: u64,
    pub venue: String,
    pub ticket_price: f64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VenueInventoryResponse {
    pub venue_id: u64,
    pub venue_name: String,
    pub total_capacity: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingRequest {
    pub user_id: u64,
    pub event_id: u64,
    pub ticket_count: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookingResponse {
    pub user_id: u64,
    pub event_id: u64,
    pub ticket_count: u64,
    pub total_price: f64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConcurrentBookingSimulationResponse {
    pub event_id: u64,
    pub initial_capacity: u64,
    pub final_capacity: u64,
    pub request_a_status: String,
    pub request_b_status: String,
    pub conclusion: String,
}
