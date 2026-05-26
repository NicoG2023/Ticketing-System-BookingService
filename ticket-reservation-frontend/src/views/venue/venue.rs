use dioxus::prelude::*;

use crate::api;
use crate::components::common::{EmptyState, LoadingState};
use crate::components::RequireRole;
use crate::models::VenueInventoryResponse;

use super::create_venue_form::CreateVenueForm;
use super::venue_card::VenueCard;

const VENUE_PAGE_CSS: Asset = asset!("/assets/styling/venue/venue-page.css");
const CREATE_VENUE_FORM_CSS: Asset = asset!("/assets/styling/venue/create-venue-form.css");
const VENUE_CARD_CSS: Asset = asset!("/assets/styling/venue/venue-card.css");
const VENUE_RESPONSIVE_CSS: Asset = asset!("/assets/styling/venue/venue-responsive.css");

#[component]
pub fn Venue() -> Element {
    rsx! {
        RequireRole {
            role: "admin".to_string(),
            VenueContent {}
        }
    }
}

#[component]
fn VenueContent() -> Element {
    let mut venues = use_signal(|| Vec::<VenueInventoryResponse>::new());

    let mut loading = use_signal(|| true);
    let mut action_loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);

    let mut reload_venues = move || {
        spawn(async move {
            loading.set(true);
            error.set(None);

            match api::get_venues().await {
                Ok(data) => venues.set(data),
                Err(message) => error.set(Some(message)),
            }

            loading.set(false);
        });
    };

    use_effect(move || {
        reload_venues();
    });

    rsx! {
        document::Link { rel: "stylesheet", href: VENUE_PAGE_CSS }
        document::Link { rel: "stylesheet", href: CREATE_VENUE_FORM_CSS }
        document::Link { rel: "stylesheet", href: VENUE_CARD_CSS }
        document::Link { rel: "stylesheet", href: VENUE_RESPONSIVE_CSS }

        main {
            class: "venue-page",

            section {
                class: "venue-header",

                div {
                    class: "venue-header__content",

                    span {
                        class: "eyebrow",
                        "Admin"
                    }

                    h1 { "Sedes" }

                    p {
                        "Registra y consulta las sedes disponibles para asociarlas a eventos."
                    }
                }

                button {
                    class: "secondary-button",
                    disabled: loading() || action_loading(),
                    onclick: move |_| {
                        reload_venues();
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

            CreateVenueForm {
                action_loading,
                on_success: move |message: String| {
                    success.set(Some(message));
                    error.set(None);
                    reload_venues();
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
                        h2 { "Sedes registradas" }
                        p { "Estas sedes pueden ser seleccionadas al crear eventos." }
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
                        message: "Crea la primera sede usando el formulario superior."
                    }
                } else {
                    div {
                        class: "venue-grid",

                        for venue in venues() {
                            VenueCard {
                                venue: venue.clone(),
                                action_loading,
                                on_success: move |message: String| {
                                    success.set(Some(message));
                                    error.set(None);
                                    reload_venues();
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
