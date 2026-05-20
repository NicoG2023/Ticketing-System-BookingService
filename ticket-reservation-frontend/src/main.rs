use dioxus::prelude::*;

mod api;
mod auth;
mod components;
mod models;
mod views;

use views::{Blog, Events, Home, Inventory, Navbar, Simulations, Venue};

#[derive(Clone)]
pub struct AuthState {
    pub initialized: Signal<bool>,
    pub authenticated: Signal<bool>,
    pub username: Signal<Option<String>>,
    pub user_id: Signal<Option<String>>,
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
        Simulations {},

        #[route("/inventory")]
        Inventory {},

        #[route("/venues")]
        Venue {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const KEYCLOAK_JS: Asset = asset!("/assets/vendor/keycloak/keycloak.js");

// Configuración de Keycloak.
// Para futuros proyectos, normalmente solo cambias estos 3 valores.
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
    let mut error = use_signal(|| None::<String>);
    let login_in_progress = use_signal(|| false);
    let notice = use_signal(|| None::<String>);

    let mut auth_bootstrap_started = use_signal(|| false);

    use_context_provider(|| AuthState {
        initialized,
        authenticated,
        username,
        user_id,
        error,
        login_in_progress,
        notice,
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
                    } else {
                        username.set(None);
                        user_id.set(None);
                    }

                    error.set(None);
                }
                Err(message) => {
                    authenticated.set(false);
                    username.set(None);
                    user_id.set(None);
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
