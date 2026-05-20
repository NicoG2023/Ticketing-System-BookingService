use dioxus::prelude::*;

use crate::api;
use crate::components::RequireAuth;
use crate::models::{BookingRequest, BookingResponse, EventInventoryResponse};
use crate::AuthState;

const EVENTS_CSS: Asset = asset!("/assets/styling/events.css");

#[component]
pub fn Events() -> Element {
    rsx! {
        RequireAuth {  }
        EventsContent {}
    }
}

#[component]
fn EventsContent() -> Element {
    let auth_state = use_context::<AuthState>();

    let mut events = use_signal(|| Vec::<EventInventoryResponse>::new());
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

    rsx! {
        document::Link { rel: "stylesheet", href: EVENTS_CSS }

        main {
            class: "events-page",

            h1 { "Eventos disponibles" }

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

                div {
                    class: "events-grid",

                    for event in events() {
                        EventCard {
                            event: event.clone(),
                            on_success: move |response: BookingResponse| {
                                success.set(Some(format!(
                                    "Reserva creada. Evento: {}, tickets: {}, total: ${}",
                                    response.event_id,
                                    response.ticket_count,
                                    response.total_price
                                )));
                            },
                            on_error: move |message: String| {
                                error.set(Some(message));
                            }
                        }
                    }
                }
        }
    }
}

#[component]
fn EventCard(
    event: EventInventoryResponse,
    on_success: EventHandler<BookingResponse>,
    on_error: EventHandler<String>,
) -> Element {
    let auth_state = use_context::<AuthState>();

    let mut ticket_count = use_signal(|| "1".to_string());
    let mut booking_loading = use_signal(|| false);

    rsx! {
        article {
            class: "event-card",

            h2 { "{event.event}" }

            p {
                strong { "Sede: " }
                "{event.venue}"
            }

            p {
                strong { "Capacidad restante: " }
                "{event.capacity}"
            }

            p {
                strong { "Precio ticket: " }
                "${event.ticket_price}"
            }

            div {
                class: "event-card__form",

                label {
                    "Tickets"
                }

                input {
                    r#type: "number",
                    min: "1",
                    value: "{ticket_count()}",
                    oninput: move |event| {
                        ticket_count.set(event.value());
                    }
                }

                button {
                    disabled: booking_loading() || event.capacity == 0,
                    onclick: move |_| {
                        let user_id_text = (auth_state.user_id)();
                        let tickets_text = ticket_count();

                        let on_success = on_success;
                        let on_error = on_error;

                        spawn(async move {
                            booking_loading.set(true);

                            let Some(user_id) = user_id_text else {
                                on_error.call("No se encontró el usuario autenticado.".to_string());
                                booking_loading.set(false);
                                return;
                            };

                            if user_id.trim().is_empty() {
                                on_error.call("El id del usuario autenticado está vacío.".to_string());
                                booking_loading.set(false);
                                return;
                            }

                            let ticket_count = match tickets_text.parse::<u64>() {
                                Ok(value) if value > 0 => value,
                                _ => {
                                    on_error.call("La cantidad de tickets debe ser mayor a cero.".to_string());
                                    booking_loading.set(false);
                                    return;
                                }
                            };

                            let request = BookingRequest {
                                user_id,
                                event_id: event.event_id,
                                ticket_count,
                            };

                            match api::create_booking(request).await {
                                Ok(response) => on_success.call(response),
                                Err(message) => on_error.call(message),
                            }

                            booking_loading.set(false);
                        });
                    },

                    if booking_loading() {
                        "Reservando..."
                    } else {
                        "Reservar"
                    }
                }
            }
        }
    }
}
