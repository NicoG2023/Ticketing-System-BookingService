use dioxus::prelude::*;

use crate::models::{BookingResponse, EventInventoryResponse};

use super::event_card::EventCard;

const EVENT_GRID_CSS: Asset = asset!("/assets/styling/events/event_grid.css");

#[component]
pub fn EventGrid(
    events: Vec<EventInventoryResponse>,
    on_success: EventHandler<BookingResponse>,
    on_error: EventHandler<String>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: EVENT_GRID_CSS }

        section {
            class: "events-grid",

            for event in events {
                EventCard {
                    key: "{event.event_id}",
                    event,
                    on_success,
                    on_error,
                }
            }
        }
    }
}
