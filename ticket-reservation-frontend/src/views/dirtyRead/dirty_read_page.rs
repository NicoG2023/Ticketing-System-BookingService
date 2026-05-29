use dioxus::prelude::*;

use crate::api;
use crate::components::RequireAuth;
use crate::models::{DirtyReadSimulationResponse, EventInventoryResponse};

use super::dirty_read_controls::{DirtyReadAction, DirtyReadControls};
use super::dirty_read_diagnosis::DirtyReadDiagnosis;
use super::dirty_read_metrics::DirtyReadMetrics;
use super::dirty_read_sessions::DirtyReadSessions;
use super::dirty_read_timeline::DirtyReadTimeline;
use crate::views::simulation::event_selector::EventSelector;

const DIRTY_READ_PAGE_CSS: Asset = asset!("/assets/styling/dirty_read/dirty_read_page.css");

#[component]
pub fn DirtyReadPage() -> Element {
    rsx! {
        RequireAuth {}
        DirtyReadContent {}
    }
}

#[component]
fn DirtyReadContent() -> Element {
    let mut events = use_signal(Vec::<EventInventoryResponse>::new);
    let mut selected_event_id = use_signal(|| None::<u64>);
    let mut loading_events = use_signal(|| true);
    let mut simulation = use_signal(|| None::<DirtyReadSimulationResponse>);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
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

    let current_step = simulation
        .read()
        .as_ref()
        .map(|state| state.current_step.clone())
        .unwrap_or_else(|| "NOT_STARTED".to_string());

    let has_selected_event = selected_event_id().is_some();

    let on_action = move |action: DirtyReadAction| {
        let Some(event_id) = selected_event_id() else {
            error.set(Some(
                "Selecciona un evento antes de ejecutar la simulación Dirty Read.".to_string(),
            ));
            return;
        };

        loading.set(true);
        error.set(None);

        spawn(async move {
            let result = execute_dirty_read_action(event_id, action).await;

            match result {
                Ok(response) => {
                    simulation.set(Some(response));
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            loading.set(false);
        });
    };

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: DIRTY_READ_PAGE_CSS
        }

        main {
            class: "dirty-read-page",

            div {
                class: "dirty-read-page__hero",

                div {
                    class: "dirty-read-page__hero-content",

                    p {
                        class: "dirty-read-page__eyebrow",
                        "Simulación de concurrencia"
                    }

                    h2 {
                        "Dirty Read"
                    }

                    p {
                        class: "dirty-read-page__description",
                        "Esta simulación muestra cómo una sesión puede leer un valor temporal no confirmado generado por otra sesión, y cómo ese valor puede desaparecer cuando se ejecuta rollback."
                    }

                    div {
                        class: "dirty-read-page__hero-note",

                        span { "Concepto clave" }

                        strong { " B lee un dato que nunca fue confirmado" }
                    }
                }
            }

            if let Some(message) = error() {
                div {
                    class: "dirty-read-page__error",
                    "{message}"
                }
            }

            section {
                class: "dirty-read-page__layout",

                div {
                    class: "dirty-read-page__left",

                    EventSelector {
                        events: events(),
                        selected_event_id: selected_event_id(),
                        loading: loading_events(),
                        on_select: move |event_id| {
                            selected_event_id.set(Some(event_id));
                            simulation.set(None);
                            error.set(None);
                        }
                    }

                    if let Some(event) = selected_event.clone() {
                        div {
                            class: "dirty-read-page__selected-event",

                            div {
                                span { "Evento seleccionado" }
                                strong { "{event.event}" }
                            }

                            div {
                                span { "Capacidad actual" }
                                strong { "{event.capacity}" }
                            }

                            div {
                                span { "Sede" }
                                strong { "{event.venue}" }
                            }
                        }
                    } else {
                        div {
                            class: "dirty-read-page__empty-state",
                            "Selecciona un evento para iniciar la simulación Dirty Read."
                        }
                    }

                    DirtyReadControls {
                        current_step,
                        has_selected_event,
                        loading: loading(),
                        on_action
                    }

                    DirtyReadSessions {
                        simulation: simulation()
                    }
                }

                div {
                    class: "dirty-read-page__right",

                    DirtyReadMetrics {
                        simulation: simulation()
                    }

                    DirtyReadTimeline {
                        simulation: simulation()
                    }

                    DirtyReadDiagnosis {
                        simulation: simulation()
                    }
                }
            }
        }
    }
}

async fn execute_dirty_read_action(
    event_id: u64,
    action: DirtyReadAction,
) -> Result<DirtyReadSimulationResponse, String> {
    match action {
        DirtyReadAction::Start => api::start_dirty_read_simulation(event_id).await,
        DirtyReadAction::StartSessionA => api::start_dirty_read_session_a(event_id).await,
        DirtyReadAction::WriteUncommitted => {
            api::write_dirty_read_uncommitted_capacity(event_id).await
        }
        DirtyReadAction::ReadBySessionB => {
            api::read_dirty_read_uncommitted_capacity_by_session_b(event_id).await
        }
        DirtyReadAction::RollbackSessionA => api::rollback_dirty_read_session_a(event_id).await,
        DirtyReadAction::ReadFinalConfirmed => {
            api::read_dirty_read_final_confirmed_capacity(event_id).await
        }
        DirtyReadAction::Diagnose => api::diagnose_dirty_read_simulation(event_id).await,
        DirtyReadAction::Reset => api::reset_dirty_read_simulation(event_id).await,
    }
}
