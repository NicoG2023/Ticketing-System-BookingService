use gloo_net::http::Request;

use crate::auth;
use crate::models::{
    BookingRequest, BookingResponse, ConcurrentBookingSimulationResponse, EventInventoryResponse,
    VenueInventoryResponse,
};

const API_BASE: &str = "/api/v1";

async fn bearer_token() -> Result<String, String> {
    let token = auth::get_token()
        .await?
        .ok_or_else(|| "No hay token de sesión. Inicia sesión nuevamente.".to_string())?;

    Ok(format!("Bearer {token}"))
}

pub async fn get_events() -> Result<Vec<EventInventoryResponse>, String> {
    let token = bearer_token().await?;

    let response = Request::get(&format!("{API_BASE}/inventory/events"))
        .header("Authorization", &token)
        .send()
        .await
        .map_err(|error| format!("Error llamando al backend: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<Vec<EventInventoryResponse>>()
        .await
        .map_err(|error| format!("Error leyendo la lista de eventos: {error}"))
}

pub async fn get_event(event_id: u64) -> Result<EventInventoryResponse, String> {
    let token = bearer_token().await?;

    let response = Request::get(&format!("{API_BASE}/inventory/event/{event_id}"))
        .header("Authorization", &token)
        .send()
        .await
        .map_err(|error| format!("Error llamando al backend: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<EventInventoryResponse>()
        .await
        .map_err(|error| format!("Error leyendo el evento: {error}"))
}

pub async fn get_venue(venue_id: u64) -> Result<VenueInventoryResponse, String> {
    let token = bearer_token().await?;

    let response = Request::get(&format!("{API_BASE}/inventory/venue/{venue_id}"))
        .header("Authorization", &token)
        .send()
        .await
        .map_err(|error| format!("Error llamando al backend: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<VenueInventoryResponse>()
        .await
        .map_err(|error| format!("Error leyendo la información del venue: {error}"))
}

pub async fn create_booking(request: BookingRequest) -> Result<BookingResponse, String> {
    let token = bearer_token().await?;

    let response = Request::post(&format!("{API_BASE}/booking"))
        .header("Authorization", &token)
        .header("Content-Type", "application/json")
        .json(&request)
        .map_err(|error| format!("Error preparando la reserva: {error}"))?
        .send()
        .await
        .map_err(|error| format!("Error creando la reserva: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<BookingResponse>()
        .await
        .map_err(|error| format!("Error leyendo la respuesta de la reserva: {error}"))
}

pub async fn simulate_concurrent_booking(
    event_id: u64,
) -> Result<ConcurrentBookingSimulationResponse, String> {
    let token = bearer_token().await?;

    let response = Request::get(&format!(
        "{API_BASE}/inventory/event/{event_id}/simulate-concurrent-booking"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error ejecutando simulación: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<ConcurrentBookingSimulationResponse>()
        .await
        .map_err(|error| format!("Error leyendo la simulación: {error}"))
}
