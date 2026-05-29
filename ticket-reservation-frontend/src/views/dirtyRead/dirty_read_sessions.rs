use dioxus::prelude::*;

use crate::models::DirtyReadSimulationResponse;

const DIRTY_READ_SESSIONS_CSS: Asset = asset!("/assets/styling/dirty_read/dirty_read_sessions.css");

#[component]
pub fn DirtyReadSessions(simulation: Option<DirtyReadSimulationResponse>) -> Element {
    let session_a_status = simulation
        .as_ref()
        .map(|state| state.session_a_status.clone())
        .unwrap_or_else(|| "Pendiente".to_string());

    let session_b_status = simulation
        .as_ref()
        .map(|state| state.session_b_status.clone())
        .unwrap_or_else(|| "Pendiente".to_string());

    let session_a_value = simulation
        .as_ref()
        .and_then(|state| state.session_a_uncommitted_capacity)
        .map(|value| value.to_string())
        .unwrap_or_else(|| "—".to_string());

    let session_b_value = simulation
        .as_ref()
        .and_then(|state| state.session_b_read_capacity)
        .map(|value| value.to_string())
        .unwrap_or_else(|| "—".to_string());

    let a_has_uncommitted = simulation
        .as_ref()
        .map(|state| state.session_a_has_uncommitted_write)
        .unwrap_or(false);

    let b_has_read = simulation
        .as_ref()
        .map(|state| state.session_b_has_read)
        .unwrap_or(false);

    let rollback_executed = simulation
        .as_ref()
        .map(|state| state.rollback_executed)
        .unwrap_or(false);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: DIRTY_READ_SESSIONS_CSS
        }

        section {
            class: "dirty-read-sessions",

            div {
                class: "dirty-read-sessions__header",
                p { "Sesiones concurrentes" }
                h3 { "Lectura no confirmada" }
            }

            div {
                class: "dirty-read-sessions__cards",

                article {
                    class: if rollback_executed {
                        "dirty-read-session dirty-read-session--a dirty-read-session--rollback"
                    } else if a_has_uncommitted {
                        "dirty-read-session dirty-read-session--a dirty-read-session--active"
                    } else {
                        "dirty-read-session dirty-read-session--a"
                    },

                    div {
                        class: "dirty-read-session__top",

                        div {
                            span {
                                class: "dirty-read-session__avatar",
                                "A"
                            }

                            div {
                                h4 { "Sesión A" }
                                p { "Operación temporal" }
                            }
                        }

                        span {
                            class: "dirty-read-session__badge",
                            if rollback_executed {
                                "ROLLBACK"
                            } else if a_has_uncommitted {
                                "NO CONFIRMADO"
                            } else {
                                "ESPERANDO"
                            }
                        }
                    }

                    div {
                        class: "dirty-read-session__value",

                        span { "Capacidad temporal" }

                        strong {
                            "{session_a_value}"
                        }
                    }

                    p {
                        class: "dirty-read-session__status",
                        "{session_a_status}"
                    }
                }

                article {
                    class: if b_has_read {
                        "dirty-read-session dirty-read-session--b dirty-read-session--danger"
                    } else {
                        "dirty-read-session dirty-read-session--b"
                    },

                    div {
                        class: "dirty-read-session__top",

                        div {
                            span {
                                class: "dirty-read-session__avatar",
                                "B"
                            }

                            div {
                                h4 { "Sesión B" }
                                p { "Lectura concurrente" }
                            }
                        }

                        span {
                            class: "dirty-read-session__badge",
                            if b_has_read {
                                "LEYÓ TEMPORAL"
                            } else {
                                "ESPERANDO"
                            }
                        }
                    }

                    div {
                        class: "dirty-read-session__value",

                        span { "Valor leído por B" }

                        strong {
                            "{session_b_value}"
                        }
                    }

                    p {
                        class: "dirty-read-session__status",
                        "{session_b_status}"
                    }
                }
            }
        }
    }
}
