use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

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
    let mut refreshing = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);
    let mut initialized = use_signal(|| false);

    let load_events = move || {
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
    };

    let refresh_events = move || {
        spawn(async move {
            refreshing.set(true);

            match api::get_events().await {
                Ok(data) => {
                    events.set(data);
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            refreshing.set(false);
        });
    };

    use_effect(move || {
        if initialized() {
            return;
        }

        initialized.set(true);
        load_events();
    });

    let handle_success = move |response: BookingResponse| {
        let booked_event_id = response.event_id;
        let booked_tickets = response.ticket_count;

        success.set(Some(format!(
            "Reserva creada. Evento: {}, tickets: {}, total: ${}",
            response.event_id, response.ticket_count, response.total_price
        )));
        error.set(None);

        /*
            Actualización optimista:
            Bajamos la capacidad local inmediatamente para que el usuario vea el cambio
            sin esperar a que Firebase/backend terminen de reflejarlo en el GET.
        */
        let mut updated_events = events();

        if let Some(event) = updated_events
            .iter_mut()
            .find(|event| event.event_id == booked_event_id)
        {
            event.capacity = event.capacity.saturating_sub(booked_tickets);
        }

        events.set(updated_events);

        /*
            Refresh real:
            Esperamos un poco y luego volvemos a consultar backend para dejar
            el frontend sincronizado con la fuente real de datos.
        */
        spawn(async move {
            TimeoutFuture::new(700).await;
            refresh_events();
        });
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

            if refreshing() {
                div {
                    class: "alert alert-warning",
                    "Actualizando disponibilidad..."
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
