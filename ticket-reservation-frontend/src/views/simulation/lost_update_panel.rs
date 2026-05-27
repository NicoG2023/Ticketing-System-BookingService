use dioxus::prelude::*;

use crate::models::EventInventoryResponse;

const LOST_UPDATE_PANEL_CSS: Asset = asset!("/assets/styling/simulation/lost_update_panel.css");

#[component]
pub fn LostUpdatePanel(
    selected_event: Option<EventInventoryResponse>,
    running: bool,
    on_run: EventHandler<MouseEvent>,
) -> Element {
    let has_selected_event = selected_event.is_some();

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: LOST_UPDATE_PANEL_CSS
        }

        section {
            class: "simulation-card lost-update-panel",

            div {
                class: "simulation-card__header",

                div {
                    p {
                        class: "simulation-card__eyebrow",
                        "Paso 2"
                    }

                    h2 {
                        "Ejecuta la simulación"
                    }
                }
            }

            div {
                class: "lost-update-panel__body",

                p {
                    "Esta prueba lanza dos operaciones concurrentes que leen la misma capacidad inicial y luego escriben el nuevo valor sin control transaccional."
                }

                if let Some(event) = selected_event.as_ref() {
                    div {
                        class: "lost-update-panel__event",

                        span {
                            "Evento seleccionado"
                        }

                        strong {
                            "{event.event}"
                        }

                        p {
                            "Capacidad actual: {event.capacity}"
                        }
                    }
                } else {
                    div {
                        class: "lost-update-panel__event lost-update-panel__event--empty",

                        "Selecciona un evento para continuar."
                    }
                }

                button {
                    class: "lost-update-panel__button",
                    disabled: running || !has_selected_event,
                    onclick: move |event| {
                        on_run.call(event);
                    },

                    if running {
                        "Simulando..."
                    } else {
                        "Simular Lost Update"
                    }
                }
            }
        }
    }
}
