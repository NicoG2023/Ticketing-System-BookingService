use dioxus::prelude::*;

use crate::models::LostUpdateSimulationResponse;

const SIMULATION_TIMELINE_CSS: Asset = asset!("/assets/styling/simulation/simulation_timeline.css");

#[component]
pub fn SimulationTimeline(loading: bool, result: Option<LostUpdateSimulationResponse>) -> Element {
    let has_result = result.is_some();

    let a_read_done = result
        .as_ref()
        .is_some_and(|response| response.request_a_read_capacity.is_some());

    let a_calculate_done = result
        .as_ref()
        .is_some_and(|response| response.request_a_calculated_capacity.is_some());

    let a_commit_done = result
        .as_ref()
        .is_some_and(|response| response.request_a_committed);

    let b_read_done = result
        .as_ref()
        .is_some_and(|response| response.request_b_read_capacity.is_some());

    let b_calculate_done = result
        .as_ref()
        .is_some_and(|response| response.request_b_calculated_capacity.is_some());

    let b_commit_done = result
        .as_ref()
        .is_some_and(|response| response.request_b_committed);

    let lost_update_occurred = result
        .as_ref()
        .is_some_and(|response| response.lost_update_occurred);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: SIMULATION_TIMELINE_CSS
        }

        section {
            class: "simulation-card simulation-timeline",

            div {
                class: "simulation-card__header",

                div {
                    p {
                        class: "simulation-card__eyebrow",
                        "Visualización"
                    }

                    h2 {
                        "Flujo concurrente"
                    }
                }
            }

            div {
                class: if loading {
                    "timeline timeline--running"
                } else if lost_update_occurred {
                    "timeline timeline--danger"
                } else if has_result {
                    "timeline timeline--done"
                } else {
                    "timeline"
                },

                div {
                    class: "timeline__lane timeline__lane--a",

                    div {
                        class: "timeline__request",
                        "Sesión A"
                    }

                    div {
                        class: step_class(a_read_done),
                        "1. Lee capacidad"
                    }

                    div {
                        class: step_class(a_calculate_done),
                        "2. Calcula nueva capacidad"
                    }

                    div {
                        class: step_class(a_commit_done),
                        "3. Escribe resultado"
                    }
                }

                div {
                    class: if lost_update_occurred {
                        "timeline__conflict timeline__conflict--active"
                    } else {
                        "timeline__conflict"
                    },

                    div {
                        class: "timeline__conflict-line"
                    }

                    div {
                        class: "timeline__conflict-badge",
                        if lost_update_occurred {
                            "Sobrescritura detectada"
                        } else {
                            "Zona de conflicto"
                        }
                    }
                }

                div {
                    class: "timeline__lane timeline__lane--b",

                    div {
                        class: "timeline__request",
                        "Sesión B"
                    }

                    div {
                        class: step_class(b_read_done),
                        "1. Lee capacidad"
                    }

                    div {
                        class: step_class(b_calculate_done),
                        "2. Calcula nueva capacidad"
                    }

                    div {
                        class: step_class(b_commit_done),
                        "3. Escribe resultado"
                    }
                }
            }

            if loading {
                p {
                    class: "timeline__hint",
                    "Procesando operación..."
                }
            } else if result.is_none() {
                p {
                    class: "timeline__hint",
                    "Inicia la simulación para ver cómo se produce el Lost Update."
                }
            } else if lost_update_occurred {
                p {
                    class: "timeline__hint timeline__hint--danger",
                    "Ambas sesiones escribieron un resultado calculado sobre una lectura antigua. Por eso una reserva se perdió."
                }
            } else {
                p {
                    class: "timeline__hint",
                    "Sigue el orden recomendado: A lee → B lee → A calcula → B calcula → A guarda → B guarda."
                }
            }
        }
    }
}

fn step_class(done: bool) -> &'static str {
    if done {
        "timeline__step timeline__step--done"
    } else {
        "timeline__step"
    }
}
