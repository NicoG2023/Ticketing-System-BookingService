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
    pub address: String,
    pub total_capacity: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVenueRequest {
    pub name: String,
    pub address: String,
    pub total_capacity: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingRequest {
    pub event_id: u64,
    pub ticket_count: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookingResponse {
    pub user_id: String,
    pub event_id: u64,
    pub ticket_count: u64,
    pub total_price: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequest {
    pub name: String,
    pub total_capacity: u64,
    pub venue_id: u64,
    pub ticket_price: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVenueRequest {
    pub name: String,
    pub address: String,
    pub total_capacity: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LostUpdateSimulationResponse {
    pub event_id: u64,

    pub initial_capacity: u64,

    pub request_a_read_capacity: Option<u64>,
    pub request_a_calculated_capacity: Option<u64>,

    pub request_b_read_capacity: Option<u64>,
    pub request_b_calculated_capacity: Option<u64>,

    pub final_capacity: Option<u64>,
    pub expected_capacity: u64,

    pub request_a_committed: bool,
    pub request_b_committed: bool,

    pub lost_update_occurred: bool,

    pub request_a_status: String,
    pub request_b_status: String,

    pub explanation: String,
    pub control: String,
}
