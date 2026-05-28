use dioxus::prelude::*;

use crate::api;
use crate::api::LostUpdateSession;
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
    let mut action_loading = use_signal(|| false);
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

    let start_simulation = move |_| {
        let Some(event_id) = selected_event_id() else {
            error.set(Some(
                "Selecciona un evento para iniciar la simulación.".to_string(),
            ));
            return;
        };

        spawn(async move {
            action_loading.set(true);
            error.set(None);
            result.set(None);

            match api::start_lost_update_simulation(event_id).await {
                Ok(response) => {
                    result.set(Some(response));
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            action_loading.set(false);
        });
    };

    let read_a = move |_| {
        run_lost_update_action(
            selected_event_id(),
            action_loading,
            error,
            result,
            LostUpdateSession::A,
            LostUpdateAction::Read,
        );
    };

    let calculate_a = move |_| {
        run_lost_update_action(
            selected_event_id(),
            action_loading,
            error,
            result,
            LostUpdateSession::A,
            LostUpdateAction::Calculate,
        );
    };

    let commit_a = move |_| {
        run_lost_update_action(
            selected_event_id(),
            action_loading,
            error,
            result,
            LostUpdateSession::A,
            LostUpdateAction::Commit,
        );
    };

    let read_b = move |_| {
        run_lost_update_action(
            selected_event_id(),
            action_loading,
            error,
            result,
            LostUpdateSession::B,
            LostUpdateAction::Read,
        );
    };

    let calculate_b = move |_| {
        run_lost_update_action(
            selected_event_id(),
            action_loading,
            error,
            result,
            LostUpdateSession::B,
            LostUpdateAction::Calculate,
        );
    };

    let commit_b = move |_| {
        run_lost_update_action(
            selected_event_id(),
            action_loading,
            error,
            result,
            LostUpdateSession::B,
            LostUpdateAction::Commit,
        );
    };

    let restore_simulation = move |_| {
        let Some(event_id) = selected_event_id() else {
            error.set(Some(
                "Selecciona un evento para restaurar la simulación.".to_string(),
            ));
            return;
        };

        spawn(async move {
            action_loading.set(true);
            error.set(None);

            match api::restore_lost_update_simulation(event_id).await {
                Ok(response) => {
                    result.set(Some(response));
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            action_loading.set(false);
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
                        loading: action_loading(),
                        result: result(),
                        on_start: start_simulation,
                        on_read_a: read_a,
                        on_calculate_a: calculate_a,
                        on_commit_a: commit_a,
                        on_read_b: read_b,
                        on_calculate_b: calculate_b,
                        on_commit_b: commit_b,
                        on_restore: restore_simulation,
                    }
                }

                div {
                    class: "simulation-layout__right",

                    SimulationTimeline {
                        loading: action_loading(),
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

#[derive(Debug, Clone, Copy)]
enum LostUpdateAction {
    Read,
    Calculate,
    Commit,
}

fn run_lost_update_action(
    selected_event_id: Option<u64>,
    mut action_loading: Signal<bool>,
    mut error: Signal<Option<String>>,
    mut result: Signal<Option<LostUpdateSimulationResponse>>,
    session: LostUpdateSession,
    action: LostUpdateAction,
) {
    let Some(event_id) = selected_event_id else {
        error.set(Some(
            "Selecciona un evento para continuar la simulación.".to_string(),
        ));
        return;
    };

    spawn(async move {
        action_loading.set(true);
        error.set(None);

        let response = match action {
            LostUpdateAction::Read => api::read_lost_update_capacity(event_id, session).await,
            LostUpdateAction::Calculate => {
                api::calculate_lost_update_capacity(event_id, session).await
            }
            LostUpdateAction::Commit => api::commit_lost_update_capacity(event_id, session).await,
        };

        match response {
            Ok(data) => {
                result.set(Some(data));
            }
            Err(message) => {
                error.set(Some(message));
            }
        }

        action_loading.set(false);
    });
}
