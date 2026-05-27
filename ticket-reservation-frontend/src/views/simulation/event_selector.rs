use dioxus::prelude::*;

use crate::models::EventInventoryResponse;

const EVENT_SELECTOR_CSS: Asset = asset!("/assets/styling/simulation/event_selector.css");

#[component]
pub fn EventSelector(
    events: Vec<EventInventoryResponse>,
    selected_event_id: Option<u64>,
    loading: bool,
    on_select: EventHandler<u64>,
) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: EVENT_SELECTOR_CSS
        }

        section {
            class: "simulation-card event-selector",

            div {
                class: "simulation-card__header",

                div {
                    p {
                        class: "simulation-card__eyebrow",
                        "Paso 1"
                    }

                    h2 {
                        "Selecciona un evento"
                    }
                }

                span {
                    class: "event-selector__badge",
                    "{events.len()} eventos"
                }
            }

            if loading {
                div {
                    class: "event-selector__loading",
                    "Cargando eventos..."
                }
            } else if events.is_empty() {
                div {
                    class: "event-selector__empty",
                    "No hay eventos disponibles para simular."
                }
            } else {
                div {
                    class: "event-selector__list",

                    for event in events {
                        button {
                            key: "{event.event_id}",
                            class: if Some(event.event_id) == selected_event_id {
                                "event-selector__item event-selector__item--active"
                            } else {
                                "event-selector__item"
                            },
                            onclick: move |_| {
                                on_select.call(event.event_id);
                            },

                            div {
                                class: "event-selector__item-main",

                                strong {
                                    "{event.event}"
                                }

                                span {
                                    "{event.venue}"
                                }
                            }

                            div {
                                class: "event-selector__capacity",

                                span {
                                    "Capacidad"
                                }

                                strong {
                                    "{event.capacity}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
