use crate::{auth, AuthState, Route};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let mut auth_state = use_context::<AuthState>();

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",

            Link {
                to: Route::Home {},
                "Home"
            }

            if auth_state.is_logged_in() {
                Link {
                    to: Route::Events {},
                    "Eventos"
                }

                Link {
                    to: Route::Simulation {},
                    "Simulación Lost Update"
                }

                Link {
                    to: Route::DirtyReadPage {},
                    "Simulación Dirty Read"
                }
            }

            if auth_state.is_admin() {
                Link {
                    to: Route::Inventory {},
                    "Inventario"
                }

                Link {
                    to: Route::Venue {},
                    "Sedes"
                }
            }

            AuthActions {}
        }

        if let Some(message) = (auth_state.notice)() {
            div {
                class: "auth-notice",

                span {
                    "{message}"
                }

                button {
                    class: "auth-notice-close",
                    onclick: move |_| {
                        auth_state.notice.set(None);
                    },
                    "Cerrar"
                }
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn AuthActions() -> Element {
    let auth_state = use_context::<AuthState>();
    let mut menu_open = use_signal(|| false);

    if !auth_state.is_ready() {
        return rsx! {
            div {
                id: "auth-actions",
                span { "Cargando sesión..." }
            }
        };
    }

    if auth_state.is_logged_in() {
        let username = auth_state.display_name();

        return rsx! {
            div {
                id: "auth-actions",
                class: "auth-menu-container",

                button {
                    class: "user-menu-button",
                    onclick: move |_| {
                        menu_open.set(!menu_open());
                    },

                    span { "Hola, {username}" }
                    span {
                        class: "chevron",
                        "▾"
                    }
                }

                if menu_open() {
                    div {
                        class: "auth-dropdown",

                        button {
                            class: "dropdown-item",
                            onclick: move |_| {
                                let mut auth_state = auth_state.clone();

                                spawn(async move {
                                    auth_state.error.set(None);

                                    if let Err(message) = auth::logout().await {
                                        auth_state.error.set(Some(message));
                                        return;
                                    }

                                    auth_state.authenticated.set(false);
                                    auth_state.username.set(None);
                                    auth_state.user_id.set(None);
                                    auth_state.roles.set(Vec::new());
                                });
                            },
                            "Cerrar sesión"
                        }
                    }
                }
            }
        };
    }

    let login_in_progress = (auth_state.login_in_progress)();

    rsx! {
        div {
            id: "auth-actions",

            if let Some(message) = (auth_state.error)() {
                span {
                    class: "error",
                    "{message}"
                }
            }

            button {
                disabled: login_in_progress,
                onclick: move |_| {
                    let mut auth_state = auth_state.clone();

                    spawn(async move {
                        auth_state.error.set(None);
                        auth_state.login_in_progress.set(true);

                        if let Err(message) = auth::login().await {
                            auth_state.error.set(Some(message));
                            auth_state.login_in_progress.set(false);
                        }
                    });
                },

                if login_in_progress {
                    "Redirigiendo..."
                } else {
                    "Iniciar sesión"
                }
            }
        }
    }
}
