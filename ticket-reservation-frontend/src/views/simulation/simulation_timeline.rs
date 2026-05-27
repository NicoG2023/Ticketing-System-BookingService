use dioxus::prelude::*;

use crate::models::LostUpdateSimulationResponse;

const SIMULATION_TIMELINE_CSS: Asset = asset!("/assets/styling/simulation/simulation_timeline.css");

#[component]
pub fn SimulationTimeline(running: bool, result: Option<LostUpdateSimulationResponse>) -> Element {
    let has_result = result.is_some();

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
                class: if running {
                    "timeline timeline--running"
                } else if has_result {
                    "timeline timeline--done"
                } else {
                    "timeline"
                },

                div {
                    class: "timeline__lane timeline__lane--a",

                    div {
                        class: "timeline__request",
                        "Request A"
                    }

                    div {
                        class: "timeline__step",
                        "Lee capacidad"
                    }

                    div {
                        class: "timeline__step",
                        "Calcula nueva capacidad"
                    }

                    div {
                        class: "timeline__step",
                        "Escribe resultado"
                    }
                }

                div {
                    class: "timeline__conflict",

                    div {
                        class: "timeline__conflict-line"
                    }

                    div {
                        class: "timeline__conflict-badge",
                        "Sobrescritura"
                    }
                }

                div {
                    class: "timeline__lane timeline__lane--b",

                    div {
                        class: "timeline__request",
                        "Request B"
                    }

                    div {
                        class: "timeline__step",
                        "Lee capacidad"
                    }

                    div {
                        class: "timeline__step",
                        "Calcula nueva capacidad"
                    }

                    div {
                        class: "timeline__step",
                        "Escribe resultado"
                    }
                }
            }

            if !running && result.is_none() {
                p {
                    class: "timeline__hint",
                    "Ejecuta la simulación para ver cómo se produce el Lost Update."
                }
            }
        }
    }
}
