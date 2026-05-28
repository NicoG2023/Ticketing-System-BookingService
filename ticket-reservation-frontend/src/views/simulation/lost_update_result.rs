use dioxus::prelude::*;

use crate::models::LostUpdateSimulationResponse;

const LOST_UPDATE_RESULT_CSS: Asset = asset!("/assets/styling/simulation/lost_update_result.css");

#[component]
pub fn LostUpdateResult(result: Option<LostUpdateSimulationResponse>) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: LOST_UPDATE_RESULT_CSS
        }

        section {
            class: "simulation-card lost-update-result",

            div {
                class: "simulation-card__header",

                div {
                    p {
                        class: "simulation-card__eyebrow",
                        "Resultado"
                    }

                    h2 {
                        "Diagnóstico de la simulación"
                    }
                }

                if let Some(response) = result.clone() {
                    span {
                        class: if response.lost_update_occurred {
                            "lost-update-result__status lost-update-result__status--danger"
                        } else {
                            "lost-update-result__status lost-update-result__status--progress"
                        },

                        if response.lost_update_occurred {
                            "Lost Update detectado"
                        } else {
                            "En progreso"
                        }
                    }
                }
            }

            if let Some(response) = result {
                div {
                    class: "lost-update-result__section",

                    div {
                        class: "lost-update-result__section-header",
                        span { "Resumen del inventario" }
                    }

                    div {
                        class: "lost-update-result__grid lost-update-result__grid--summary",

                        div {
                            class: "lost-update-result__metric lost-update-result__metric--summary lost-update-result__metric--filled",
                            span { "Capacidad inicial" }
                            strong { "{response.initial_capacity}" }
                        }

                        div {
                            class: "lost-update-result__metric lost-update-result__metric--summary lost-update-result__metric--filled",
                            span { "Capacidad esperada" }
                            strong { "{response.expected_capacity}" }
                        }

                        div {
                            class: metric_class("summary", response.final_capacity.is_some()),
                            span { "Capacidad final" }
                            strong {
                                "{format_optional_u64(response.final_capacity)}"
                            }
                        }

                        div {
                            class: metric_class("summary", response.final_capacity.is_some()),
                            span { "Diferencia" }
                            strong {
                                "{format_difference(response.final_capacity, response.expected_capacity)}"
                            }
                        }
                    }
                }

                div {
                    class: "lost-update-result__section",

                    div {
                        class: "lost-update-result__section-header",
                        span { "Lecturas y cálculos por sesión" }
                    }

                    div {
                        class: "lost-update-result__grid lost-update-result__grid--sessions",

                        div {
                            class: metric_class("session-a", response.request_a_read_capacity.is_some()),
                            span { "Sesión A leyó" }
                            strong {
                                "{format_optional_u64(response.request_a_read_capacity)}"
                            }
                        }

                        div {
                            class: metric_class("session-a", response.request_a_calculated_capacity.is_some()),
                            span { "Sesión A calculó" }
                            strong {
                                "{format_optional_u64(response.request_a_calculated_capacity)}"
                            }
                        }

                        div {
                            class: metric_class("session-b", response.request_b_read_capacity.is_some()),
                            span { "Sesión B leyó" }
                            strong {
                                "{format_optional_u64(response.request_b_read_capacity)}"
                            }
                        }

                        div {
                            class: metric_class("session-b", response.request_b_calculated_capacity.is_some()),
                            span { "Sesión B calculó" }
                            strong {
                                "{format_optional_u64(response.request_b_calculated_capacity)}"
                            }
                        }
                    }
                }

                div {
                    class: "lost-update-result__details",

                    div {
                        class: "lost-update-result__log lost-update-result__log--a",
                        span { "Request A" }
                        p { "{response.request_a_status}" }
                    }

                    div {
                        class: "lost-update-result__log lost-update-result__log--b",
                        span { "Request B" }
                        p { "{response.request_b_status}" }
                    }
                }

                div {
                    class: if response.lost_update_occurred {
                        "lost-update-result__explanation lost-update-result__explanation--danger"
                    } else {
                        "lost-update-result__explanation"
                    },

                    h3 {
                        "¿Qué ocurrió?"
                    }

                    p {
                        "{response.explanation}"
                    }

                    h3 {
                        "¿Cómo se controla?"
                    }

                    p {
                        "{response.control}"
                    }
                }
            } else {
                div {
                    class: "lost-update-result__empty",

                    h3 {
                        "Aún no hay resultados"
                    }

                    p {
                        "Selecciona un evento e inicia la simulación para generar el diagnóstico."
                    }
                }
            }
        }
    }
}

fn format_optional_u64(value: Option<u64>) -> String {
    value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_difference(final_capacity: Option<u64>, expected_capacity: u64) -> String {
    final_capacity
        .map(|capacity| capacity as i64 - expected_capacity as i64)
        .map(|difference| difference.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn metric_class(kind: &'static str, filled: bool) -> String {
    let state_class = if filled {
        "lost-update-result__metric--filled"
    } else {
        "lost-update-result__metric--pending"
    };

    format!(
        "lost-update-result__metric lost-update-result__metric--{} {}",
        kind, state_class
    )
}
