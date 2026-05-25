use dioxus::prelude::*;

use crate::api;
use crate::models::EventInventoryResponse;

#[component]
pub fn InventoryCard(
    event: EventInventoryResponse,
    action_loading: Signal<bool>,
    on_success: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let mut amount = use_signal(|| "1".to_string());
    let mut confirm_delete = use_signal(|| false);
    let mut expanded = use_signal(|| false);

    rsx! {
        article {
            class: if expanded() { "event-row event-row--expanded" } else { "event-row" },

            div {
                class: "event-row__summary",

                div {
                    class: "event-row__main",

                    h3 { "{event.event}" }

                    p {
                        "ID {event.event_id} · {event.venue}"
                    }
                }

                div {
                    class: "event-row__metrics",

                    span {
                        class: "event-row__capacity",
                        "{event.capacity} disponibles"
                    }

                    span {
                        class: "event-row__price",
                        "${event.ticket_price}"
                    }

                    button {
                        class: "manage-button",
                        disabled: action_loading(),
                        onclick: move |_| {
                            expanded.set(!expanded());
                            confirm_delete.set(false);
                        },

                        if expanded() {
                            "Cerrar"
                        } else {
                            "Gestionar"
                        }
                    }
                }
            }

            if expanded() {
                div {
                    class: "event-row__details",

                    div {
                        class: "event-row__detail-grid",

                        div {
                            class: "event-detail-item",
                            span { "Sede" }
                            strong { "{event.venue}" }
                        }

                        div {
                            class: "event-detail-item",
                            span { "Precio" }
                            strong { "${event.ticket_price}" }
                        }

                        div {
                            class: "event-detail-item",
                            span { "Disponibles" }
                            strong { "{event.capacity}" }
                        }
                    }

                    div {
                        class: "event-row__actions-panel",

                        div {
                            class: "event-row__actions-copy",

                            h4 { "Ajustar inventario" }
                            p { "Ingresa la cantidad de tickets que quieres descontar o liberar." }
                        }

                        div {
                            class: "event-row__actions-form",

                            label {
                                class: "quantity-field",

                                span { "Cantidad" }

                                input {
                                    r#type: "number",
                                    min: "1",
                                    value: "{amount()}",
                                    oninput: move |evt| {
                                        amount.set(evt.value());
                                    }
                                }
                            }

                            div {
                                class: "event-row__action-buttons",

                                button {
                                    class: "action-button action-button--decrease",
                                    disabled: action_loading(),
                                    onclick: move |_| {
                                        let amount_text = amount();
                                        let on_success = on_success;
                                        let on_error = on_error;
                                        let event_id = event.event_id;

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

                                            match api::decrease_event_capacity(event_id, quantity).await {
                                                Ok(_) => {
                                                    on_success.call(format!(
                                                        "Se descontaron {} tickets del evento {}.",
                                                        quantity,
                                                        event_id
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
                                    class: "action-button action-button--release",
                                    disabled: action_loading(),
                                    onclick: move |_| {
                                        let amount_text = amount();
                                        let on_success = on_success;
                                        let on_error = on_error;
                                        let event_id = event.event_id;

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

                                            match api::release_event_capacity(event_id, quantity).await {
                                                Ok(_) => {
                                                    on_success.call(format!(
                                                        "Se liberaron {} tickets del evento {}.",
                                                        quantity,
                                                        event_id
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

                    div {
                        class: "event-row__danger-zone",

                        if confirm_delete() {
                            div {
                                class: "delete-confirmation",

                                div {
                                    h4 { "Eliminar evento" }
                                    p { "Esta acción eliminará el evento del inventario." }
                                }

                                div {
                                    class: "delete-confirmation__actions",

                                    button {
                                        class: "danger-solid-button",
                                        disabled: action_loading(),
                                        onclick: move |_| {
                                            let on_success = on_success;
                                            let on_error = on_error;
                                            let event_id = event.event_id;

                                            spawn(async move {
                                                action_loading.set(true);

                                                match api::delete_event(event_id).await {
                                                    Ok(_) => {
                                                        on_success.call(format!(
                                                            "Evento {} eliminado correctamente.",
                                                            event_id
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
                                            "Eliminando..."
                                        } else {
                                            "Sí, eliminar"
                                        }
                                    }

                                    button {
                                        class: "neutral-button",
                                        disabled: action_loading(),
                                        onclick: move |_| {
                                            confirm_delete.set(false);
                                        },
                                        "Cancelar"
                                    }
                                }
                            }
                        } else {
                            button {
                                class: "danger-text-button",
                                disabled: action_loading(),
                                onclick: move |_| {
                                    confirm_delete.set(true);
                                },
                                "Eliminar evento"
                            }
                        }
                    }
                }
            }
        }
    }
}
