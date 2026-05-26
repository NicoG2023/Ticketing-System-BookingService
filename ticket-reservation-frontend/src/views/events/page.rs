use dioxus::prelude::*;

use crate::api;
use crate::components::RequireAuth;
use crate::models::{BookingResponse, EventInventoryResponse};

use super::event_grid::EventGrid;

const EVENTS_PAGE_CSS: Asset = asset!("/assets/styling/events/page.css");

#[component]
pub fn Events() -> Element {
    rsx! {
        RequireAuth {}
        EventsContent {}
    }
}

#[component]
fn EventsContent() -> Element {
    let mut events = use_signal(Vec::<EventInventoryResponse>::new);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);

    use_effect(move || {
        spawn(async move {
            loading.set(true);
            error.set(None);

            match api::get_events().await {
                Ok(data) => {
                    events.set(data);
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            loading.set(false);
        });
    });

    let handle_success = move |response: BookingResponse| {
        success.set(Some(format!(
            "Reserva creada. Evento: {}, tickets: {}, total: ${}",
            response.event_id, response.ticket_count, response.total_price
        )));
        error.set(None);
    };

    let handle_error = move |message: String| {
        error.set(Some(message));
        success.set(None);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: EVENTS_PAGE_CSS }

        main {
            class: "events-page",

            section {
                class: "events-hero",

                p {
                    class: "eyebrow",
                    "Sistema de reservas"
                }

                h1 { "Eventos disponibles" }

                p {
                    class: "events-hero__description",
                    "Selecciona un evento, elige la cantidad de tickets y confirma tu reserva."
                }
            }

            if let Some(message) = error() {
                div {
                    class: "alert alert-error",
                    "{message}"
                }
            }

            if let Some(message) = success() {
                div {
                    class: "alert alert-success",
                    "{message}"
                }
            }

            if loading() {
                div {
                    class: "events-loading",
                    "Cargando eventos..."
                }
            } else if events().is_empty() {
                div {
                    class: "events-empty",
                    h2 { "No hay eventos disponibles" }
                    p { "Cuando se registren eventos, aparecerán en esta sección." }
                }
            } else {
                EventGrid {
                    events: events(),
                    on_success: handle_success,
                    on_error: handle_error,
                }
            }
        }
    }
}
