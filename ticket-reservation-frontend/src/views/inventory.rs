use dioxus::prelude::*;

use crate::api;
use crate::models::EventInventoryResponse;
use crate::AuthState;

const INVENTORY_CSS: Asset = asset!("/assets/styling/inventory.css");

#[component]
pub fn Inventory() -> Element {
    let auth_state = use_context::<AuthState>();

    let mut events = use_signal(|| Vec::<EventInventoryResponse>::new());
    let mut loading = use_signal(|| true);
    let mut action_loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);

    let mut reload_inventory = move || {
        spawn(async move {
            loading.set(true);
            error.set(None);

            match api::get_events().await {
                Ok(data) => {
                    events.set(data);
                }
                Err(message) => {
                    error.set(Some(message));
                }
            }

            loading.set(false);
        });
    };

    use_effect(move || {
        reload_inventory();
    });

    rsx! {
        document::Link { rel: "stylesheet", href: INVENTORY_CSS }

        main {
            class: "inventory-page",

            div {
                class: "inventory-header",

                div {
                    h1 { "Gestión de inventario" }
                    p {
                        "Consulta eventos, revisa capacidad disponible y ejecuta operaciones de descuento o liberación de inventario."
                    }
                }

                button {
                    class: "secondary-button",
                    disabled: loading() || action_loading(),
                    onclick: move |_| {
                        reload_inventory();
                    },

                    if loading() {
                        "Cargando..."
                    } else {
                        "Refrescar"
                    }
                }
            }

            if !auth_state.is_logged_in() {
                div {
                    class: "alert alert-warning",
                    "Debes iniciar sesión para gestionar el inventario."
                }
            } else {
                if let Some(message) = error() {
                    div {
                        class: "alert alert-error",
                        "{message}"
                    }
                }

                if let Some(message) = success() {
                    div {
                        class: "alert alert-success",
                        "{message}"
                    }
                }

                if loading() {
                    p { "Cargando inventario..." }
                } else if events().is_empty() {
                    div {
                        class: "empty-state",
                        h2 { "No hay eventos registrados" }
                        p { "Cuando cargues eventos en Firebase, aparecerán en esta pantalla." }
                    }
                } else {
                    div {
                        class: "inventory-grid",

                        for event in events() {
                            InventoryCard {
                                event: event.clone(),
                                action_loading,
                                on_success: move |message: String| {
                                    success.set(Some(message));
                                    error.set(None);
                                    reload_inventory();
                                },
                                on_error: move |message: String| {
                                    error.set(Some(message));
                                    success.set(None);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InventoryCard(
    event: EventInventoryResponse,
    action_loading: Signal<bool>,
    on_success: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let mut amount = use_signal(|| "1".to_string());

    rsx! {
        article {
            class: "inventory-card",

            div {
                class: "inventory-card__header",

                div {
                    h2 { "{event.event}" }
                    p { "Evento ID: {event.event_id}" }
                }

                span {
                    class: "capacity-badge",
                    "{event.capacity} disponibles"
                }
            }

            div {
                class: "inventory-card__details",

                p {
                    strong { "Sede: " }
                    "{event.venue}"
                }

                p {
                    strong { "Precio: " }
                    "${event.ticket_price}"
                }
            }

            div {
                class: "inventory-card__actions",

                label {
                    "Cantidad"
                }

                input {
                    r#type: "number",
                    min: "1",
                    value: "{amount()}",
                    oninput: move |evt| {
                        amount.set(evt.value());
                    }
                }

                div {
                    class: "button-row",

                    button {
                        class: "danger-button",
                        disabled: action_loading(),
                        onclick: move |_| {
                            let amount_text = amount();
                            let on_success = on_success;
                            let on_error = on_error;

                            spawn(async move {
                                action_loading.set(true);

                                let quantity = match amount_text.parse::<u64>() {
                                    Ok(value) if value > 0 => value,
                                    _ => {
                                        on_error.call("La cantidad debe ser mayor a cero.".to_string());
                                        action_loading.set(false);
                                        return;
                                    }
                                };

                                match api::decrease_event_capacity(event.event_id, quantity).await {
                                    Ok(_) => {
                                        on_success.call(format!(
                                            "Se descontaron {} tickets del evento {}.",
                                            quantity,
                                            event.event_id
                                        ));
                                    }
                                    Err(message) => {
                                        on_error.call(message);
                                    }
                                }

                                action_loading.set(false);
                            });
                        },

                        if action_loading() {
                            "Procesando..."
                        } else {
                            "Descontar"
                        }
                    }

                    button {
                        class: "success-button",
                        disabled: action_loading(),
                        onclick: move |_| {
                            let amount_text = amount();
                            let on_success = on_success;
                            let on_error = on_error;

                            spawn(async move {
                                action_loading.set(true);

                                let quantity = match amount_text.parse::<u64>() {
                                    Ok(value) if value > 0 => value,
                                    _ => {
                                        on_error.call("La cantidad debe ser mayor a cero.".to_string());
                                        action_loading.set(false);
                                        return;
                                    }
                                };

                                match api::release_event_capacity(event.event_id, quantity).await {
                                    Ok(_) => {
                                        on_success.call(format!(
                                            "Se liberaron {} tickets del evento {}.",
                                            quantity,
                                            event.event_id
                                        ));
                                    }
                                    Err(message) => {
                                        on_error.call(message);
                                    }
                                }

                                action_loading.set(false);
                            });
                        },

                        if action_loading() {
                            "Procesando..."
                        } else {
                            "Liberar"
                        }
                    }
                }
            }
        }
    }
}
