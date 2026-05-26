use dioxus::prelude::*;

use crate::models::{BookingResponse, EventInventoryResponse};

use super::booking_form::BookingForm;

const EVENT_CARD_CSS: Asset = asset!("/assets/styling/events/event_card.css");

#[component]
pub fn EventCard(
    event: EventInventoryResponse,
    on_success: EventHandler<BookingResponse>,
    on_error: EventHandler<String>,
) -> Element {
    let formatted_price = format!("{:.2}", event.ticket_price);

    rsx! {
        document::Link { rel: "stylesheet", href: EVENT_CARD_CSS }

        article {
            class: "event-card",

            div {
                class: "event-card__header",

                div {
                    p {
                        class: "event-card__venue",
                        "{event.venue}"
                    }

                    h2 {
                        class: "event-card__title",
                        "{event.event}"
                    }
                }

                span {
                    class: if event.capacity == 0 {
                        "event-card__status event-card__status--sold-out"
                    } else {
                        "event-card__status"
                    },

                    if event.capacity == 0 {
                        "Agotado"
                    } else {
                        "Disponible"
                    }
                }
            }

            div {
                class: "event-card__details",

                div {
                    class: "event-card__metric",
                    span { "Capacidad restante" }
                    strong { "{event.capacity}" }
                }

                div {
                    class: "event-card__metric",
                    span { "Precio ticket" }
                    strong { "${formatted_price}" }
                }
            }

            BookingForm {
                event_id: event.event_id,
                available_capacity: event.capacity,
                on_success,
                on_error,
            }
        }
    }
}
