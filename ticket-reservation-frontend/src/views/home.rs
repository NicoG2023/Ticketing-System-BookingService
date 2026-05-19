use dioxus::prelude::*;

use crate::{auth, AuthState, Route};

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    let auth_state = use_context::<AuthState>();

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        main {
            class: "home-page",

            section {
                class: "home-hero",

                div {
                    class: "home-hero__content",

                    span {
                        class: "home-hero__badge",
                        "Sistema de reservas"
                    }

                    h1 {
                        class: "home-hero__title",
                        "Reserva tus tickets de forma rápida y segura"
                    }

                    p {
                        class: "home-hero__description",
                        "Consulta eventos disponibles, revisa su capacidad y crea reservas usando una aplicación conectada a microservicios con API Gateway y Keycloak."
                    }

                    div {
                        class: "home-hero__actions",

                        Link {
                            class: "home-hero__primary-button",
                            to: Route::Events {},
                            "Ver eventos"
                        }

                        if !(auth_state.initialized)() {
                            button {
                                class: "home-hero__secondary-button",
                                disabled: true,
                                "Cargando sesión..."
                            }
                        } else if !(auth_state.authenticated)() {
                            button {
                                class: "home-hero__secondary-button",
                                onclick: move |_| {
                                    auth::login();
                                },
                                "Iniciar sesión"
                            }
                        } else {
                            span {
                                class: "home-hero__user",
                                "Sesión iniciada como {display_username(&auth_state)}"
                            }
                        }
                    }
                }

                div {
                    class: "home-hero__panel",

                    div {
                        class: "home-summary-card",
                        span {
                            class: "home-summary-card__label",
                            "Paso 1"
                        }
                        h3 { "Explora eventos" }
                        p { "Consulta la lista de eventos publicados desde el servicio de inventario." }
                    }

                    div {
                        class: "home-summary-card",
                        span {
                            class: "home-summary-card__label",
                            "Paso 2"
                        }
                        h3 { "Selecciona tickets" }
                        p { "Elige la cantidad de tickets disponibles para el evento." }
                    }

                    div {
                        class: "home-summary-card",
                        span {
                            class: "home-summary-card__label",
                            "Paso 3"
                        }
                        h3 { "Confirma reserva" }
                        p { "La reserva se crea a través del API Gateway y se actualiza el inventario." }
                    }
                }
            }
        }
    }
}

fn display_username(auth_state: &AuthState) -> String {
    (auth_state.username)().unwrap_or_else(|| "usuario".to_string())
}
