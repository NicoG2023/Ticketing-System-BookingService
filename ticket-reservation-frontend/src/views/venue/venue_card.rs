use dioxus::prelude::*;

use crate::api;
use crate::models::VenueInventoryResponse;

#[component]
pub fn VenueCard(
    venue: VenueInventoryResponse,
    action_loading: Signal<bool>,
    on_success: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let mut confirm_delete = use_signal(|| false);

    rsx! {
        article {
            class: "venue-card",

            div {
                class: "venue-card__top",

                div {
                    h3 { "{venue.venue_name}" }
                    p { "ID {venue.venue_id}" }
                }

                span {
                    class: "capacity-badge",
                    "{venue.total_capacity}"
                }
            }

            div {
                class: "venue-card__details",

                p {
                    span { "Dirección" }
                    strong { "{venue.address}" }
                }

                p {
                    span { "Capacidad" }
                    strong { "{venue.total_capacity}" }
                }
            }

            div {
                class: "venue-card__actions delete-zone",

                if confirm_delete() {
                    p {
                        class: "delete-warning",
                        "¿Seguro que quieres eliminar esta sede?"
                    }

                    p {
                        class: "delete-help",
                        "Solo se podrá eliminar si no tiene eventos asociados."
                    }

                    div {
                        class: "button-row",

                        button {
                            class: "ghost-button danger",
                            disabled: action_loading(),
                            onclick: move |_| {
                                let on_success = on_success;
                                let on_error = on_error;
                                let venue_id = venue.venue_id;

                                spawn(async move {
                                    action_loading.set(true);

                                    match api::delete_venue(venue_id).await {
                                        Ok(_) => {
                                            on_success.call(format!(
                                                "Sede {} eliminada correctamente.",
                                                venue_id
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
                            class: "ghost-button",
                            disabled: action_loading(),
                            onclick: move |_| {
                                confirm_delete.set(false);
                            },
                            "Cancelar"
                        }
                    }
                } else {
                    button {
                        class: "ghost-button danger delete-button",
                        disabled: action_loading(),
                        onclick: move |_| {
                            confirm_delete.set(true);
                        },
                        "Eliminar sede"
                    }
                }
            }
        }
    }
}
