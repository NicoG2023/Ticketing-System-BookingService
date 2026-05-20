use crate::{AuthState, Route};
use dioxus::prelude::*;

#[component]
pub fn RequireAuth(children: Element) -> Element {
    let auth_state = use_context::<AuthState>();
    let nav = use_navigator();

    let is_ready = auth_state.is_ready();
    let is_logged_in = auth_state.is_logged_in();

    use_effect({
        let mut auth_state = auth_state.clone();
        let nav = nav.clone();

        move || {
            if !auth_state.is_ready() {
                return;
            }

            if !auth_state.is_logged_in() {
                auth_state.notice.set(Some(
                    "Debes iniciar sesión para acceder a esa página.".to_string(),
                ));

                nav.replace(Route::Home {});
            }
        }
    });

    if !is_ready {
        return rsx! {
            div {
                "Verificando sesión..."
            }
        };
    }

    if !is_logged_in {
        return rsx! {
            div {
                "Redirigiendo..."
            }
        };
    }

    rsx! {
        {children}
    }
}
