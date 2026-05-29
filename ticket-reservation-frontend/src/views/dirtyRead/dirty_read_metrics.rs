use dioxus::prelude::*;

use crate::models::DirtyReadSimulationResponse;

const DIRTY_READ_METRICS_CSS: Asset = asset!("/assets/styling/dirty_read/dirty_read_metrics.css");

#[component]
pub fn DirtyReadMetrics(simulation: Option<DirtyReadSimulationResponse>) -> Element {
    let initial = simulation
        .as_ref()
        .map(|state| state.initial_confirmed_capacity.to_string())
        .unwrap_or_else(|| "—".to_string());

    let temporal = simulation
        .as_ref()
        .and_then(|state| state.session_a_uncommitted_capacity)
        .map(|value| value.to_string())
        .unwrap_or_else(|| "—".to_string());

    let read_by_b = simulation
        .as_ref()
        .and_then(|state| state.session_b_read_capacity)
        .map(|value| value.to_string())
        .unwrap_or_else(|| "—".to_string());

    let final_confirmed = simulation
        .as_ref()
        .and_then(|state| state.final_confirmed_capacity)
        .map(|value| value.to_string())
        .unwrap_or_else(|| "—".to_string());

    let rollback = simulation
        .as_ref()
        .map(|state| if state.rollback_executed { "Sí" } else { "No" })
        .unwrap_or("No");

    let dirty_read = simulation
        .as_ref()
        .map(|state| {
            if state.dirty_read_detected {
                "Sí"
            } else {
                "No"
            }
        })
        .unwrap_or("No");

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: DIRTY_READ_METRICS_CSS
        }

        section {
            class: "dirty-read-metrics",

            div {
                class: "dirty-read-metrics__header",
                p { "Métricas" }
                h3 { "Evidencia del Dirty Read" }
            }

            div {
                class: "dirty-read-metrics__grid",

                MetricCard {
                    label: "Capacidad inicial",
                    value: initial,
                    hint: "Valor confirmado en Firebase",
                    variant: "neutral"
                }

                MetricCard {
                    label: "Temporal de A",
                    value: temporal,
                    hint: "Valor no confirmado",
                    variant: "warning"
                }

                MetricCard {
                    label: "Leído por B",
                    value: read_by_b,
                    hint: "Lectura potencialmente sucia",
                    variant: "danger"
                }

                MetricCard {
                    label: "Final confirmado",
                    value: final_confirmed,
                    hint: "Valor real después del rollback",
                    variant: "success"
                }

                MetricCard {
                    label: "Rollback",
                    value: rollback.to_string(),
                    hint: "A descartó el cambio temporal",
                    variant: "warning"
                }

                MetricCard {
                    label: "Dirty Read",
                    value: dirty_read.to_string(),
                    hint: "Anomalía detectada",
                    variant: "danger"
                }
            }
        }
    }
}

#[component]
fn MetricCard(
    label: &'static str,
    value: String,
    hint: &'static str,
    variant: &'static str,
) -> Element {
    rsx! {
        article {
            class: "dirty-read-metric dirty-read-metric--{variant}",

            span {
                class: "dirty-read-metric__label",
                "{label}"
            }

            strong {
                class: "dirty-read-metric__value",
                "{value}"
            }

            small {
                class: "dirty-read-metric__hint",
                "{hint}"
            }
        }
    }
}
