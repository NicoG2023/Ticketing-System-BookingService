use dioxus::prelude::*;

use crate::models::DirtyReadSimulationResponse;

const DIRTY_READ_TIMELINE_CSS: Asset = asset!("/assets/styling/dirty_read/dirty_read_timeline.css");

#[component]
pub fn DirtyReadTimeline(simulation: Option<DirtyReadSimulationResponse>) -> Element {
    let current_step = simulation
        .as_ref()
        .map(|state| state.current_step.clone())
        .unwrap_or_else(|| "NOT_STARTED".to_string());

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: DIRTY_READ_TIMELINE_CSS
        }

        section {
            class: "dirty-read-timeline",

            div {
                class: "dirty-read-timeline__header",
                p { "Línea de tiempo" }
                h3 { "Cómo aparece la lectura sucia" }
            }

            div {
                class: "dirty-read-timeline__items",

                TimelineItem {
                    number: "1",
                    step: "INITIALIZED",
                    current_step: current_step.clone(),
                    title: "Se lee capacidad inicial",
                    description: "El backend consulta Firebase y toma la capacidad confirmada como punto de partida.",
                    actor: "Sistema"
                }

                TimelineItem {
                    number: "2",
                    step: "SESSION_A_STARTED",
                    current_step: current_step.clone(),
                    title: "Sesión A inicia",
                    description: "A abre una operación temporal. Todavía no hay escritura confirmada.",
                    actor: "Sesión A"
                }

                TimelineItem {
                    number: "3",
                    step: "SESSION_A_UNCOMMITTED_WRITE",
                    current_step: current_step.clone(),
                    title: "A escribe valor temporal",
                    description: "A calcula una capacidad menor, pero ese dato queda solo en memoria.",
                    actor: "Sesión A"
                }

                TimelineItem {
                    number: "4",
                    step: "SESSION_B_DIRTY_READ",
                    current_step: current_step.clone(),
                    title: "B lee el valor temporal",
                    description: "B observa un valor que todavía no ha sido confirmado.",
                    actor: "Sesión B"
                }

                TimelineItem {
                    number: "5",
                    step: "SESSION_A_ROLLBACK",
                    current_step: current_step.clone(),
                    title: "A ejecuta rollback",
                    description: "El valor temporal se descarta y nunca llega a Firebase.",
                    actor: "Sesión A"
                }

                TimelineItem {
                    number: "6",
                    step: "FINAL_CONFIRMED_READ",
                    current_step: current_step.clone(),
                    title: "Se consulta el valor final",
                    description: "Firebase sigue teniendo la capacidad confirmada original.",
                    actor: "Sistema"
                }

                TimelineItem {
                    number: "7",
                    step: "COMPLETED",
                    current_step: current_step.clone(),
                    title: "Diagnóstico final",
                    description: "Se compara lo leído por B contra el valor final confirmado.",
                    actor: "Sistema"
                }
            }
        }
    }
}

#[component]
fn TimelineItem(
    number: &'static str,
    step: &'static str,
    current_step: String,
    title: &'static str,
    description: &'static str,
    actor: &'static str,
) -> Element {
    let status_class = timeline_status_class(&current_step, step);

    rsx! {
        article {
            class: "dirty-read-timeline__item {status_class}",

            div {
                class: "dirty-read-timeline__marker",
                "{number}"
            }

            div {
                class: "dirty-read-timeline__content",

                div {
                    class: "dirty-read-timeline__item-top",

                    h4 { "{title}" }

                    span { "{actor}" }
                }

                p { "{description}" }
            }
        }
    }
}

fn timeline_status_class(current_step: &str, item_step: &str) -> &'static str {
    let current_index = step_index(current_step);
    let item_index = step_index(item_step);

    if current_step == "NOT_STARTED" {
        return "";
    }

    if item_index < current_index {
        "dirty-read-timeline__item--completed"
    } else if item_index == current_index {
        "dirty-read-timeline__item--active"
    } else {
        ""
    }
}

fn step_index(step: &str) -> usize {
    match step {
        "NOT_STARTED" => 0,
        "INITIALIZED" => 1,
        "SESSION_A_STARTED" => 2,
        "SESSION_A_UNCOMMITTED_WRITE" => 3,
        "SESSION_B_DIRTY_READ" => 4,
        "SESSION_A_ROLLBACK" => 5,
        "FINAL_CONFIRMED_READ" => 6,
        "COMPLETED" => 7,
        _ => 0,
    }
}
