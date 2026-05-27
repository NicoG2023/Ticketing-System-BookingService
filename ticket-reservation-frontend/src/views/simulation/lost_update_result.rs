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
                            "lost-update-result__status lost-update-result__status--success"
                        },

                        if response.lost_update_occurred {
                            "Lost Update detectado"
                        } else {
                            "Sin pérdida"
                        }
                    }
                }
            }

            if let Some(response) = result {
                div {
                    class: "lost-update-result__grid",

                    div {
                        class: "lost-update-result__metric",
                        span { "Capacidad inicial" }
                        strong { "{response.initial_capacity}" }
                    }

                    div {
                        class: "lost-update-result__metric",
                        span { "Capacidad esperada" }
                        strong { "{response.expected_capacity}" }
                    }

                    div {
                        class: "lost-update-result__metric",
                        span { "Capacidad final" }
                        strong { "{response.final_capacity}" }
                    }

                    div {
                        class: "lost-update-result__metric",
                        span { "Diferencia" }
                        strong {
                            "{response.final_capacity as i64 - response.expected_capacity as i64}"
                        }
                    }
                }

                div {
                    class: "lost-update-result__details",

                    div {
                        class: "lost-update-result__log",
                        span { "Request A" }
                        p { "{response.request_a_status}" }
                    }

                    div {
                        class: "lost-update-result__log",
                        span { "Request B" }
                        p { "{response.request_b_status}" }
                    }
                }

                div {
                    class: "lost-update-result__explanation",

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
                        "Selecciona un evento y ejecuta la simulación para generar el diagnóstico."
                    }
                }
            }
        }
    }
}
