use dioxus::prelude::*;

use crate::api;
use crate::components::RequireAuth;
use crate::models::{CreateVenueRequest, VenueInventoryResponse};

const VENUE_CSS: Asset = asset!("/assets/styling/venue.css");

#[component]
pub fn Venue() -> Element {
    rsx! {
        RequireAuth {
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

    let mut venue_name = use_signal(|| String::new());
    let mut venue_address = use_signal(|| String::new());
    let mut venue_capacity = use_signal(|| "1000".to_string());

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
        document::Link { rel: "stylesheet", href: VENUE_CSS }

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

            section {
                class: "panel",

                div {
                    class: "panel-header",

                    div {
                        h2 { "Registrar sede" }
                        p { "Crea una sede que luego podrá usarse al registrar eventos." }
                    }
                }

                div {
                    class: "create-venue-form",

                    div {
                        class: "form-field",
                        label { "Nombre" }

                        input {
                            placeholder: "Movistar Arena",
                            value: "{venue_name()}",
                            oninput: move |evt| {
                                venue_name.set(evt.value());
                            }
                        }
                    }

                    div {
                        class: "form-field",
                        label { "Dirección" }

                        input {
                            placeholder: "Bogotá, Colombia",
                            value: "{venue_address()}",
                            oninput: move |evt| {
                                venue_address.set(evt.value());
                            }
                        }
                    }

                    div {
                        class: "form-field",
                        label { "Capacidad total" }

                        input {
                            r#type: "number",
                            min: "1",
                            value: "{venue_capacity()}",
                            oninput: move |evt| {
                                venue_capacity.set(evt.value());
                            }
                        }
                    }

                    button {
                        class: "primary-button",
                        disabled: action_loading(),
                        onclick: move |_| {
                            let name = venue_name();
                            let address = venue_address();
                            let capacity_text = venue_capacity();

                            spawn(async move {
                                action_loading.set(true);
                                error.set(None);
                                success.set(None);

                                if name.trim().is_empty() {
                                    error.set(Some("El nombre de la sede es obligatorio.".to_string()));
                                    action_loading.set(false);
                                    return;
                                }

                                if address.trim().is_empty() {
                                    error.set(Some("La dirección de la sede es obligatoria.".to_string()));
                                    action_loading.set(false);
                                    return;
                                }

                                let total_capacity = match capacity_text.parse::<u64>() {
                                    Ok(value) if value > 0 => value,
                                    _ => {
                                        error.set(Some("La capacidad debe ser mayor a cero.".to_string()));
                                        action_loading.set(false);
                                        return;
                                    }
                                };

                                let request = CreateVenueRequest {
                                    name,
                                    address,
                                    total_capacity,
                                };

                                match api::create_venue(request).await {
                                    Ok(created_venue) => {
                                        success.set(Some(format!(
                                            "Sede creada: {}",
                                            created_venue.venue_name
                                        )));

                                        venue_name.set(String::new());
                                        venue_address.set(String::new());
                                        venue_capacity.set("1000".to_string());

                                        reload_venues();
                                    }
                                    Err(message) => {
                                        error.set(Some(message));
                                    }
                                }

                                action_loading.set(false);
                            });
                        },

                        if action_loading() {
                            "Guardando..."
                        } else {
                            "Crear sede"
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
                        p { "Estas sedes pueden ser seleccionadas al crear eventos." }
                    }

                    span {
                        class: "count-badge",
                        "{venues().len()} sedes"
                    }
                }

                if loading() {
                    p {
                        class: "muted",
                        "Cargando sedes..."
                    }
                } else if venues().is_empty() {
                    div {
                        class: "empty-state",
                        h3 { "No hay sedes registradas" }
                        p { "Crea la primera sede usando el formulario superior." }
                    }
                } else {
                    div {
                        class: "venue-grid",

                        for venue in venues() {
                            VenueCard {
                                venue: venue.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VenueCard(venue: VenueInventoryResponse) -> Element {
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
                    span { "Capacidad" }
                    strong { "{venue.total_capacity}" }
                }
            }
        }
    }
}
