use dioxus::prelude::*;

use crate::models::{EventInventoryResponse, LostUpdateSimulationResponse};

const LOST_UPDATE_PANEL_CSS: Asset = asset!("/assets/styling/simulation/lost_update_panel.css");

#[component]
pub fn LostUpdatePanel(
    selected_event: Option<EventInventoryResponse>,
    loading: bool,
    result: Option<LostUpdateSimulationResponse>,
    on_start: EventHandler<MouseEvent>,
    on_read_a: EventHandler<MouseEvent>,
    on_calculate_a: EventHandler<MouseEvent>,
    on_commit_a: EventHandler<MouseEvent>,
    on_read_b: EventHandler<MouseEvent>,
    on_calculate_b: EventHandler<MouseEvent>,
    on_commit_b: EventHandler<MouseEvent>,
    on_restore: EventHandler<MouseEvent>,
) -> Element {
    let has_selected_event = selected_event.is_some();
    let has_started = result.is_some();

    let can_read_a = has_started && !loading;

    let can_calculate_a = result
        .as_ref()
        .is_some_and(|response| response.request_a_read_capacity.is_some())
        && !loading;

    let can_commit_a = result
        .as_ref()
        .is_some_and(|response| response.request_a_calculated_capacity.is_some())
        && !loading;

    let can_read_b = has_started && !loading;

    let can_calculate_b = result
        .as_ref()
        .is_some_and(|response| response.request_b_read_capacity.is_some())
        && !loading;

    let can_commit_b = result
        .as_ref()
        .is_some_and(|response| response.request_b_calculated_capacity.is_some())
        && !loading;

    let request_a_status = result
        .as_ref()
        .map(|response| response.request_a_status.clone())
        .unwrap_or_else(|| "Pendiente".to_string());

    let request_b_status = result
        .as_ref()
        .map(|response| response.request_b_status.clone())
        .unwrap_or_else(|| "Pendiente".to_string());

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
                        "Controla la simulación"
                    }
                }
            }

            div {
                class: "lost-update-panel__body",

                p {
                    "Ejecuta manualmente las operaciones de dos sesiones. Para provocar el Lost Update, haz que A y B lean antes de que cualquiera guarde."
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

                div {
                    class: "lost-update-panel__actions",

                    button {
                        class: "lost-update-panel__button",
                        disabled: loading || !has_selected_event,
                        onclick: move |event| {
                            on_start.call(event);
                        },

                        if loading {
                            "Procesando..."
                        } else if has_started {
                            "Reiniciar simulación"
                        } else {
                            "Iniciar simulación"
                        }
                    }

                    button {
                        class: "lost-update-panel__button lost-update-panel__button--secondary",
                        disabled: loading || !has_started,
                        onclick: move |event| {
                            on_restore.call(event);
                        },
                        "Restaurar capacidad"
                    }
                }

                div {
                    class: "lost-update-panel__sessions",

                    div {
                        class: "lost-update-panel__session",

                        h3 {
                            "Sesión A"
                        }

                        p {
                            class: "lost-update-panel__session-status",
                            "{request_a_status}"
                        }

                        button {
                            class: "lost-update-panel__step-button",
                            disabled: !can_read_a,
                            onclick: move |event| {
                                on_read_a.call(event);
                            },
                            "1. Leer capacidad"
                        }

                        button {
                            class: "lost-update-panel__step-button",
                            disabled: !can_calculate_a,
                            onclick: move |event| {
                                on_calculate_a.call(event);
                            },
                            "2. Calcular reserva"
                        }

                        button {
                            class: "lost-update-panel__step-button",
                            disabled: !can_commit_a,
                            onclick: move |event| {
                                on_commit_a.call(event);
                            },
                            "3. Guardar resultado"
                        }
                    }

                    div {
                        class: "lost-update-panel__session",

                        h3 {
                            "Sesión B"
                        }

                        p {
                            class: "lost-update-panel__session-status",
                            "{request_b_status}"
                        }

                        button {
                            class: "lost-update-panel__step-button",
                            disabled: !can_read_b,
                            onclick: move |event| {
                                on_read_b.call(event);
                            },
                            "1. Leer capacidad"
                        }

                        button {
                            class: "lost-update-panel__step-button",
                            disabled: !can_calculate_b,
                            onclick: move |event| {
                                on_calculate_b.call(event);
                            },
                            "2. Calcular reserva"
                        }

                        button {
                            class: "lost-update-panel__step-button",
                            disabled: !can_commit_b,
                            onclick: move |event| {
                                on_commit_b.call(event);
                            },
                            "3. Guardar resultado"
                        }
                    }
                }

                div {
                    class: "lost-update-panel__hint",

                    strong {
                        "Orden recomendado:"
                    }

                    p {
                        "A lee → B lee → A calcula → B calcula → A guarda → B guarda."
                    }
                }
            }
        }
    }
}
