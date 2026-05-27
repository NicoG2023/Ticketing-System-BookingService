use dioxus::prelude::*;

use crate::api;
use crate::components::RequireAuth;
use crate::models::{EventInventoryResponse, LostUpdateSimulationResponse};

use super::event_selector::EventSelector;
use super::lost_update_panel::LostUpdatePanel;
use super::lost_update_result::LostUpdateResult;
use super::simulation_header::SimulationHeader;
use super::simulation_timeline::SimulationTimeline;

const SIMULATION_PAGE_CSS: Asset = asset!("/assets/styling/simulation/page.css");

#[component]
pub fn Simulation() -> Element {
    rsx! {
        RequireAuth {}
        SimulationContent {}
    }
}

#[component]
fn SimulationContent() -> Element {
    let mut events = use_signal(Vec::<EventInventoryResponse>::new);
    let mut selected_event_id = use_signal(|| None::<u64>);
    let mut loading_events = use_signal(|| true);
    let mut running_simulation = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut result = use_signal(|| None::<LostUpdateSimulationResponse>);
    let mut initialized = use_signal(|| false);

    let load_events = move || {
        spawn(async move {
            loading_events.set(true);
            error.set(None);

            match api::get_events().await {
                Ok(data) => {
                    if let Some(first_event) = data.first() {
                        selected_event_id.set(Some(first_event.event_id));
                    }

                    events.set(data);
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            loading_events.set(false);
        });
    };

    use_effect(move || {
        if initialized() {
            return;
        }

        initialized.set(true);
        load_events();
    });

    let selected_event = events()
        .into_iter()
        .find(|event| Some(event.event_id) == selected_event_id());

    let run_lost_update_simulation = move |_| {
        let Some(event_id) = selected_event_id() else {
            error.set(Some(
                "Selecciona un evento para ejecutar la simulación.".to_string(),
            ));
            return;
        };

        spawn(async move {
            running_simulation.set(true);
            error.set(None);
            result.set(None);

            match api::simulate_lost_update(event_id).await {
                Ok(response) => {
                    result.set(Some(response));
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            running_simulation.set(false);
        });
    };

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: SIMULATION_PAGE_CSS
        }

        main {
            class: "simulation-page",

            SimulationHeader {}

            if let Some(message) = error() {
                div {
                    class: "simulation-alert simulation-alert--error",
                    "{message}"
                }
            }

            section {
                class: "simulation-layout",

                div {
                    class: "simulation-layout__left",

                    EventSelector {
                        events: events(),
                        selected_event_id: selected_event_id(),
                        loading: loading_events(),
                        on_select: move |event_id| {
                            selected_event_id.set(Some(event_id));
                            result.set(None);
                            error.set(None);
                        }
                    }

                    LostUpdatePanel {
                        selected_event,
                        running: running_simulation(),
                        on_run: run_lost_update_simulation,
                    }
                }

                div {
                    class: "simulation-layout__right",

                    SimulationTimeline {
                        running: running_simulation(),
                        result: result(),
                    }

                    LostUpdateResult {
                        result: result(),
                    }
                }
            }
        }
    }
}
