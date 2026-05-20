use dioxus::prelude::*;

use crate::api;
use crate::components::RequireAuth;
use crate::models::ConcurrentBookingSimulationResponse;

const SIMULATIONS_CSS: Asset = asset!("/assets/styling/simulations.css");

#[component]
pub fn Simulations() -> Element {
    rsx! {
        RequireAuth {
            SimulationsContent {}
        }
    }
}

#[component]
fn SimulationsContent() -> Element {
    let mut event_id = use_signal(|| "1".to_string());
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut result = use_signal(|| None::<ConcurrentBookingSimulationResponse>);

    rsx! {
        document::Link { rel: "stylesheet", href: SIMULATIONS_CSS }

        main {
            class: "simulations-page",

            h1 { "Simulaciones de Base de Datos" }

            p {
                class: "simulations-description",
                "Esta sección permite evidenciar problemas de concurrencia y control transaccional usando Firebase Realtime Database."
            }

            section {
                class: "simulation-card",

                h2 { "Simulación 1: Doble reserva / Lost Update" }

                p {
                    "Esta simulación lanza dos reservas concurrentes sobre el mismo evento. Para verla bien, deja la capacidad del evento en 1 antes de ejecutarla."
                }

                div {
                    class: "simulation-form",

                    label { "ID del evento" }

                    input {
                        r#type: "number",
                        min: "1",
                        value: "{event_id()}",
                        oninput: move |event| {
                            event_id.set(event.value());
                        }
                    }

                    button {
                        disabled: loading(),
                        onclick: move |_| {
                            let event_id_text = event_id();

                            spawn(async move {
                                loading.set(true);
                                error.set(None);
                                result.set(None);

                                let event_id = match event_id_text.parse::<u64>() {
                                    Ok(value) => value,
                                    Err(_) => {
                                        error.set(Some("El id del evento debe ser numérico.".to_string()));
                                        loading.set(false);
                                        return;
                                    }
                                };

                                match api::simulate_concurrent_booking(event_id).await {
                                    Ok(response) => result.set(Some(response)),
                                    Err(message) => error.set(Some(message)),
                                }

                                loading.set(false);
                            });
                        },

                        if loading() {
                            "Ejecutando..."
                        } else {
                            "Ejecutar simulación"
                        }
                    }
                }

                if let Some(message) = error() {
                    div {
                        class: "alert alert-error",
                        "{message}"
                    }
                }

                if let Some(simulation) = result() {
                    SimulationResult { simulation }
                }
            }
        }
    }
}

#[component]
fn SimulationResult(simulation: ConcurrentBookingSimulationResponse) -> Element {
    rsx! {
        div {
            class: "simulation-result",

            h3 { "Resultado" }

            table {
                tbody {
                    tr {
                        td { "Evento" }
                        td { "{simulation.event_id}" }
                    }

                    tr {
                        td { "Capacidad inicial" }
                        td { "{simulation.initial_capacity}" }
                    }

                    tr {
                        td { "Solicitud A" }
                        td { "{simulation.request_a_status}" }
                    }

                    tr {
                        td { "Solicitud B" }
                        td { "{simulation.request_b_status}" }
                    }

                    tr {
                        td { "Capacidad final" }
                        td { "{simulation.final_capacity}" }
                    }
                }
            }

            div {
                class: "simulation-conclusion",

                strong { "Conclusión: " }
                "{simulation.conclusion}"
            }
        }
    }
}
