use crate::{AuthState, Route};
use dioxus::prelude::*;

#[component]
pub fn RequireRole(role: String, children: Element) -> Element {
    let auth_state = use_context::<AuthState>();
    let nav = use_navigator();

    let is_ready = auth_state.is_ready();
    let is_logged_in = auth_state.is_logged_in();
    let has_role = auth_state.has_role(&role);

    use_effect({
        let mut auth_state = auth_state.clone();
        let nav = nav.clone();
        let role = role.clone();

        move || {
            if !auth_state.is_ready() {
                return;
            }

            if !auth_state.is_logged_in() {
                auth_state.notice.set(Some(
                    "Debes iniciar sesión para acceder a esa página.".to_string(),
                ));

                nav.replace(Route::Home {});
                return;
            }

            if !auth_state.has_role(&role) {
                auth_state.notice.set(Some(
                    "No tienes permisos para acceder a esa página.".to_string(),
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

    if !has_role {
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
