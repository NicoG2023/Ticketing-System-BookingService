use gloo_net::http::Request;

use crate::auth;
use crate::models::{
    BookingRequest, BookingResponse, CreateEventRequest, CreateVenueRequest,
    EventInventoryResponse, LostUpdateSimulationResponse, UpdateVenueRequest,
    VenueInventoryResponse,
};

const API_BASE: &str = "http://localhost:8091/api/v1";

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

pub async fn decrease_event_capacity(event_id: u64, capacity: u64) -> Result<(), String> {
    let token = bearer_token().await?;

    let response = Request::put(&format!(
        "{API_BASE}/inventory/event/{event_id}/capacity/{capacity}"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error descontando inventario: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    Ok(())
}

pub async fn release_event_capacity(event_id: u64, tickets_released: u64) -> Result<(), String> {
    let token = bearer_token().await?;

    let response = Request::put(&format!(
        "{API_BASE}/inventory/event/{event_id}/capacity/release/{tickets_released}"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error liberando inventario: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    Ok(())
}

pub async fn create_event(request: CreateEventRequest) -> Result<EventInventoryResponse, String> {
    let token = bearer_token().await?;

    let response = Request::post(&format!("{API_BASE}/inventory/events"))
        .header("Authorization", &token)
        .header("Content-Type", "application/json")
        .json(&request)
        .map_err(|error| format!("Error preparando el evento: {error}"))?
        .send()
        .await
        .map_err(|error| format!("Error creando el evento: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<EventInventoryResponse>()
        .await
        .map_err(|error| format!("Error leyendo el evento creado: {error}"))
}

pub async fn get_venues() -> Result<Vec<VenueInventoryResponse>, String> {
    let token = bearer_token().await?;

    let response = Request::get(&format!("{API_BASE}/inventory/venues"))
        .header("Authorization", &token)
        .send()
        .await
        .map_err(|error| format!("Error llamando al backend: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<Vec<VenueInventoryResponse>>()
        .await
        .map_err(|error| format!("Error leyendo la lista de sedes: {error}"))
}

pub async fn create_venue(request: CreateVenueRequest) -> Result<VenueInventoryResponse, String> {
    let token = bearer_token().await?;

    let response = Request::post(&format!("{API_BASE}/inventory/venues"))
        .header("Authorization", &token)
        .header("Content-Type", "application/json")
        .json(&request)
        .map_err(|error| format!("Error preparando la sede: {error}"))?
        .send()
        .await
        .map_err(|error| format!("Error creando la sede: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<VenueInventoryResponse>()
        .await
        .map_err(|error| format!("Error leyendo la sede creada: {error}"))
}

pub async fn update_venue(
    venue_id: u64,
    request: UpdateVenueRequest,
) -> Result<VenueInventoryResponse, String> {
    let token = bearer_token().await?;

    let response = Request::put(&format!("{API_BASE}/inventory/venue/{venue_id}"))
        .header("Authorization", &token)
        .header("Content-Type", "application/json")
        .json(&request)
        .map_err(|error| format!("Error preparando la actualización de la sede: {error}"))?
        .send()
        .await
        .map_err(|error| format!("Error actualizando la sede: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<VenueInventoryResponse>()
        .await
        .map_err(|error| format!("Error leyendo la sede actualizada: {error}"))
}

pub async fn delete_event(event_id: u64) -> Result<(), String> {
    let token = bearer_token().await?;

    let response = Request::delete(&format!("{API_BASE}/inventory/event/{event_id}"))
        .header("Authorization", &token)
        .send()
        .await
        .map_err(|error| format!("Error eliminando el evento: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    Ok(())
}

pub async fn delete_venue(venue_id: u64) -> Result<(), String> {
    let token = bearer_token().await?;

    let response = Request::delete(&format!("{API_BASE}/inventory/venue/{venue_id}"))
        .header("Authorization", &token)
        .send()
        .await
        .map_err(|error| format!("Error eliminando la sede: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum LostUpdateSession {
    A,
    B,
}

impl LostUpdateSession {
    fn as_str(&self) -> &'static str {
        match self {
            LostUpdateSession::A => "A",
            LostUpdateSession::B => "B",
        }
    }
}

pub async fn start_lost_update_simulation(
    event_id: u64,
) -> Result<LostUpdateSimulationResponse, String> {
    let token = bearer_token().await?;

    let response = Request::post(&format!(
        "{API_BASE}/events/{event_id}/simulations/lost-update/start"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error iniciando simulación Lost Update: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<LostUpdateSimulationResponse>()
        .await
        .map_err(|error| format!("Error leyendo la simulación Lost Update: {error}"))
}

pub async fn read_lost_update_capacity(
    event_id: u64,
    session: LostUpdateSession,
) -> Result<LostUpdateSimulationResponse, String> {
    let token = bearer_token().await?;
    let session = session.as_str();

    let response = Request::post(&format!(
        "{API_BASE}/events/{event_id}/simulations/lost-update/sessions/{session}/read"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error leyendo capacidad en simulación: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<LostUpdateSimulationResponse>()
        .await
        .map_err(|error| format!("Error leyendo la respuesta de simulación: {error}"))
}

pub async fn calculate_lost_update_capacity(
    event_id: u64,
    session: LostUpdateSession,
) -> Result<LostUpdateSimulationResponse, String> {
    let token = bearer_token().await?;
    let session = session.as_str();

    let response = Request::post(&format!(
        "{API_BASE}/events/{event_id}/simulations/lost-update/sessions/{session}/calculate"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error calculando capacidad en simulación: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<LostUpdateSimulationResponse>()
        .await
        .map_err(|error| format!("Error leyendo la respuesta de simulación: {error}"))
}

pub async fn commit_lost_update_capacity(
    event_id: u64,
    session: LostUpdateSession,
) -> Result<LostUpdateSimulationResponse, String> {
    let token = bearer_token().await?;
    let session = session.as_str();

    let response = Request::post(&format!(
        "{API_BASE}/events/{event_id}/simulations/lost-update/sessions/{session}/commit"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error guardando capacidad en simulación: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<LostUpdateSimulationResponse>()
        .await
        .map_err(|error| format!("Error leyendo la respuesta de simulación: {error}"))
}

pub async fn restore_lost_update_simulation(
    event_id: u64,
) -> Result<LostUpdateSimulationResponse, String> {
    let token = bearer_token().await?;

    let response = Request::post(&format!(
        "{API_BASE}/events/{event_id}/simulations/lost-update/restore"
    ))
    .header("Authorization", &token)
    .send()
    .await
    .map_err(|error| format!("Error restaurando simulación Lost Update: {error}"))?;

    if !response.ok() {
        return Err(format!("Error del servidor: {}", response.status()));
    }

    response
        .json::<LostUpdateSimulationResponse>()
        .await
        .map_err(|error| format!("Error leyendo la simulación restaurada: {error}"))
}
