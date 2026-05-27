use dioxus::prelude::*;
use gloo_events::EventListener;
use std::rc::Rc;
use web_sys::window as browser_window;

mod api;
mod auth;
mod components;
mod models;
mod views;

use views::{Blog, Events, Home, Inventory, Navbar, Simulation, Venue};

#[derive(Clone)]
pub struct AuthState {
    pub initialized: Signal<bool>,
    pub authenticated: Signal<bool>,
    pub username: Signal<Option<String>>,
    pub user_id: Signal<Option<String>>,
    pub roles: Signal<Vec<String>>,
    pub error: Signal<Option<String>>,
    pub login_in_progress: Signal<bool>,
    pub notice: Signal<Option<String>>,
}

impl AuthState {
    pub fn display_name(&self) -> String {
        (self.username)().unwrap_or_else(|| "usuario".to_string())
    }

    pub fn is_ready(&self) -> bool {
        (self.initialized)()
    }

    pub fn is_logged_in(&self) -> bool {
        (self.authenticated)()
    }

    pub fn has_role(&self, role: &str) -> bool {
        (self.roles)().iter().any(|user_role| user_role == role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: i32 },

        #[route("/events")]
        Events {},

        #[route("/simulations")]
        Simulation {},

        #[route("/inventory")]
        Inventory {},

        #[route("/venues")]
        Venue {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const KEYCLOAK_JS: Asset = asset!("/assets/vendor/keycloak/keycloak.js");
const KEYCLOAK_URL: &str = "http://localhost:8092";
const KEYCLOAK_REALM: &str = "ticket-reservation";
const KEYCLOAK_CLIENT_ID: &str = "ticket-frontend";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut initialized = use_signal(|| false);
    let mut authenticated = use_signal(|| false);
    let mut username = use_signal(|| None::<String>);
    let mut user_id = use_signal(|| None::<String>);
    let mut roles = use_signal(|| Vec::<String>::new());
    let mut error = use_signal(|| None::<String>);
    let login_in_progress = use_signal(|| false);
    let notice = use_signal(|| None::<String>);

    let mut auth_bootstrap_started = use_signal(|| false);

    use_context_provider(|| AuthState {
        initialized,
        authenticated,
        username,
        user_id,
        roles,
        error,
        login_in_progress,
        notice,
    });

    let _session_expired_listener = use_hook(move || {
        let Some(window) = browser_window() else {
            return None;
        };

        let mut authenticated = authenticated;
        let mut username = username;
        let mut user_id = user_id;
        let mut roles = roles;
        let mut error = error;
        let mut notice = notice;

        Some(Rc::new(EventListener::new(
            &window,
            "ticket-auth-session-expired",
            move |_| {
                authenticated.set(false);
                username.set(None);
                user_id.set(None);
                roles.set(Vec::new());
                error.set(None);
                notice.set(Some(
                    "Tu sesión expiró. Inicia sesión de nuevo para continuar.".to_string(),
                ));

                if let Some(window) = browser_window() {
                    let _ = window.location().set_href("/");
                }
            },
        )))
    });

    use_effect(move || {
        if auth_bootstrap_started() {
            return;
        }

        auth_bootstrap_started.set(true);

        let keycloak_js_url = KEYCLOAK_JS.to_string();

        spawn(async move {
            auth::set_keycloak_js_url(&keycloak_js_url);
            auth::configure_keycloak(KEYCLOAK_URL, KEYCLOAK_REALM, KEYCLOAK_CLIENT_ID);

            match auth::init_keycloak().await {
                Ok(is_authenticated) => {
                    authenticated.set(is_authenticated);

                    if is_authenticated {
                        username.set(auth::get_username());
                        user_id.set(auth::get_user_id());
                        roles.set(auth::get_client_roles());
                    } else {
                        username.set(None);
                        user_id.set(None);
                        roles.set(Vec::new());
                    }

                    error.set(None);
                }
                Err(message) => {
                    authenticated.set(false);
                    username.set(None);
                    user_id.set(None);
                    roles.set(Vec::new());
                    error.set(Some(message));
                }
            }

            initialized.set(true);
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        if initialized() {
            Router::<Route> {}
        } else {
            div {
                id: "app-loading",
                "Cargando sesión..."
            }
        }
    }
}
