use dioxus::prelude::*;

use crate::api;
use crate::components::common::{EmptyState, LoadingState};
use crate::components::RequireRole;
use crate::models::{EventInventoryResponse, VenueInventoryResponse};

use super::create_event_form::CreateEventForm;
use super::event_card::InventoryCard;
use super::venue_card::VenueCard;

const INVENTORY_PAGE_CSS: Asset = asset!("/assets/styling/inventory/inventory-page.css");
const INVENTORY_PANEL_CSS: Asset = asset!("/assets/styling/inventory/inventory-panel.css");
const CREATE_EVENT_FORM_CSS: Asset = asset!("/assets/styling/inventory/create-event-form.css");
const EVENT_CARD_CSS: Asset = asset!("/assets/styling/inventory/event-card.css");
const VENUE_CARD_CSS: Asset = asset!("/assets/styling/inventory/venue-card.css");
const INVENTORY_RESPONSIVE_CSS: Asset =
    asset!("/assets/styling/inventory/inventory-responsive.css");

#[component]
pub fn Inventory() -> Element {
    rsx! {
        RequireRole {
            role: "admin".to_string(),
            InventoryContent {}
        }
    }
}

#[component]
fn InventoryContent() -> Element {
    let mut events = use_signal(|| Vec::<EventInventoryResponse>::new());
    let mut venues = use_signal(|| Vec::<VenueInventoryResponse>::new());

    let mut loading = use_signal(|| true);
    let mut action_loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);

    let mut event_search = use_signal(|| String::new());
    let mut current_page = use_signal(|| 1usize);

    let page_size = 10usize;

    let mut reload_inventory = move || {
        spawn(async move {
            loading.set(true);
            error.set(None);

            let events_result = api::get_events().await;
            let venues_result = api::get_venues().await;

            match events_result {
                Ok(data) => events.set(data),
                Err(message) => error.set(Some(message)),
            }

            match venues_result {
                Ok(data) => venues.set(data),
                Err(message) => error.set(Some(message)),
            }

            loading.set(false);
        });
    };

    use_effect(move || {
        reload_inventory();
    });

    let search_text = event_search().trim().to_lowercase();

    let filtered_events = events()
        .iter()
        .filter(|event| {
            if search_text.is_empty() {
                return true;
            }

            event.event.to_lowercase().contains(&search_text)
                || event.venue.to_lowercase().contains(&search_text)
                || event.event_id.to_string().contains(&search_text)
        })
        .cloned()
        .collect::<Vec<_>>();

    let total_filtered_events = filtered_events.len();

    let total_pages = if total_filtered_events == 0 {
        1
    } else {
        (total_filtered_events + page_size - 1) / page_size
    };

    if current_page() > total_pages {
        current_page.set(total_pages);
    }

    let start_index = (current_page().saturating_sub(1)) * page_size;

    let paginated_events = filtered_events
        .iter()
        .skip(start_index)
        .take(page_size)
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        document::Link { rel: "stylesheet", href: INVENTORY_PAGE_CSS }
        document::Link { rel: "stylesheet", href: INVENTORY_PANEL_CSS }
        document::Link { rel: "stylesheet", href: CREATE_EVENT_FORM_CSS }
        document::Link { rel: "stylesheet", href: EVENT_CARD_CSS }
        document::Link { rel: "stylesheet", href: VENUE_CARD_CSS }
        document::Link { rel: "stylesheet", href: INVENTORY_RESPONSIVE_CSS }

        main {
            class: "inventory-page",

            section {
                class: "inventory-header",

                div {
                    class: "inventory-header__content",

                    span {
                        class: "eyebrow",
                        "Admin"
                    }

                    h1 { "Inventario" }

                    p {
                        "Registra eventos, consulta la capacidad disponible y administra el inventario del sistema."
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

            CreateEventForm {
                venues,
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

            section {
                class: "panel",

                div {
                    class: "panel-header",

                    div {
                        h2 { "Eventos registrados" }
                        p { "Busca, revisa y gestiona la capacidad de cada evento." }
                    }

                    span {
                        class: "count-badge",
                        "{total_filtered_events} de {events().len()} eventos"
                    }
                }

                div {
                    class: "inventory-toolbar",

                    div {
                        class: "inventory-search",

                        span {
                            class: "inventory-search__icon",
                            "⌕"
                        }

                        input {
                            placeholder: "Buscar por evento, sede o ID...",
                            value: "{event_search()}",
                            oninput: move |evt| {
                                event_search.set(evt.value());
                                current_page.set(1);
                            }
                        }
                    }

                    div {
                        class: "inventory-toolbar__meta",
                        "Mostrando máximo {page_size} por página"
                    }
                }

                if loading() {
                    LoadingState {
                        message: "Cargando inventario..."
                    }
                } else if events().is_empty() {
                    EmptyState {
                        title: "No hay eventos registrados",
                        message: "Crea el primer evento usando el formulario superior."
                    }
                } else if filtered_events.is_empty() {
                    EmptyState {
                        title: "No encontramos eventos",
                        message: "Intenta buscar por otro nombre, sede o ID."
                    }
                } else {
                    div {
                        class: "event-list",

                        for event in paginated_events {
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

                    div {
                        class: "pagination",

                        button {
                            class: "pagination-button",
                            disabled: current_page() <= 1,
                            onclick: move |_| {
                                current_page.set(current_page().saturating_sub(1));
                            },
                            "Anterior"
                        }

                        span {
                            class: "pagination-info",
                            "Página {current_page()} de {total_pages}"
                        }

                        button {
                            class: "pagination-button",
                            disabled: current_page() >= total_pages,
                            onclick: move |_| {
                                current_page.set(current_page() + 1);
                            },
                            "Siguiente"
                        }
                    }
                }
            }

            section {
                class: "panel",

                div {
                    class: "panel-header",

                    div {
                        h2 { "Sedes registradas" }
                        p { "Consulta y elimina sedes que no tengan eventos asociados." }
                    }

                    span {
                        class: "count-badge",
                        "{venues().len()} sedes"
                    }
                }

                if loading() {
                    LoadingState {
                        message: "Cargando sedes..."
                    }
                } else if venues().is_empty() {
                    EmptyState {
                        title: "No hay sedes registradas",
                        message: "Cuando crees sedes, aparecerán en esta sección."
                    }
                } else {
                    div {
                        class: "inventory-grid",

                        for venue in venues() {
                            VenueCard {
                                venue: venue.clone(),
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
