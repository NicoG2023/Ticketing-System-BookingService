use dioxus::prelude::*;

use crate::api;
use crate::models::{CreateEventRequest, VenueInventoryResponse};

#[component]
pub fn CreateEventForm(
    venues: Signal<Vec<VenueInventoryResponse>>,
    action_loading: Signal<bool>,
    on_success: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let mut event_name = use_signal(|| String::new());
    let mut event_capacity = use_signal(|| "100".to_string());
    let mut event_price = use_signal(|| "150000".to_string());
    let mut selected_venue_id = use_signal(|| String::new());

    use_effect(move || {
        if selected_venue_id().is_empty() {
            if let Some(first_venue) = venues().first() {
                selected_venue_id.set(first_venue.venue_id.to_string());
            }
        }
    });

    rsx! {
        section {
            class: "panel",

            div {
                class: "panel-header",

                div {
                    h2 { "Registrar evento" }
                    p { "Crea un evento asociado a una sede existente." }
                }
            }

            div {
                class: "create-event-form",

                div {
                    class: "form-field",
                    label { "Nombre" }

                    input {
                        placeholder: "Concierto de prueba",
                        value: "{event_name()}",
                        oninput: move |evt| {
                            event_name.set(evt.value());
                        }
                    }
                }

                div {
                    class: "form-field",
                    label { "Capacidad" }

                    input {
                        r#type: "number",
                        min: "1",
                        value: "{event_capacity()}",
                        oninput: move |evt| {
                            event_capacity.set(evt.value());
                        }
                    }
                }

                div {
                    class: "form-field",
                    label { "Sede" }

                    if venues().is_empty() {
                        select {
                            disabled: true,
                            option {
                                value: "",
                                "No hay sedes registradas"
                            }
                        }
                    } else {
                        select {
                            value: "{selected_venue_id()}",
                            onchange: move |evt| {
                                selected_venue_id.set(evt.value());
                            },

                            for venue in venues() {
                                option {
                                    value: "{venue.venue_id}",
                                    "{venue.venue_name}"
                                }
                            }
                        }
                    }
                }

                div {
                    class: "form-field",
                    label { "Precio" }

                    input {
                        r#type: "number",
                        min: "1",
                        value: "{event_price()}",
                        oninput: move |evt| {
                            event_price.set(evt.value());
                        }
                    }
                }

                button {
                    class: "primary-button",
                    disabled: action_loading() || venues().is_empty(),
                    onclick: move |_| {
                        let name = event_name();
                        let capacity_text = event_capacity();
                        let venue_id_text = selected_venue_id();
                        let price_text = event_price();
                        let on_success = on_success;
                        let on_error = on_error;

                        spawn(async move {
                            action_loading.set(true);

                            if name.trim().is_empty() {
                                on_error.call("El nombre del evento es obligatorio.".to_string());
                                action_loading.set(false);
                                return;
                            }

                            let total_capacity = match capacity_text.parse::<u64>() {
                                Ok(value) if value > 0 => value,
                                _ => {
                                    on_error.call("La capacidad debe ser mayor a cero.".to_string());
                                    action_loading.set(false);
                                    return;
                                }
                            };

                            if venue_id_text.trim().is_empty() {
                                on_error.call("Debes seleccionar una sede.".to_string());
                                action_loading.set(false);
                                return;
                            }

                            let venue_id = match venue_id_text.parse::<u64>() {
                                Ok(value) if value > 0 => value,
                                _ => {
                                    on_error.call("La sede seleccionada no es válida.".to_string());
                                    action_loading.set(false);
                                    return;
                                }
                            };

                            let ticket_price = match price_text.parse::<f64>() {
                                Ok(value) if value > 0.0 => value,
                                _ => {
                                    on_error.call("El precio debe ser mayor a cero.".to_string());
                                    action_loading.set(false);
                                    return;
                                }
                            };

                            let request = CreateEventRequest {
                                name,
                                total_capacity,
                                venue_id,
                                ticket_price,
                            };

                            match api::create_event(request).await {
                                Ok(created_event) => {
                                    on_success.call(format!(
                                        "Evento creado: {}",
                                        created_event.event
                                    ));

                                    event_name.set(String::new());
                                    event_capacity.set("100".to_string());
                                    event_price.set("150000".to_string());
                                }
                                Err(message) => {
                                    on_error.call(message);
                                }
                            }

                            action_loading.set(false);
                        });
                    },

                    if action_loading() {
                        "Guardando..."
                    } else if venues().is_empty() {
                        "Sin sedes"
                    } else {
                        "Crear evento"
                    }
                }
            }
        }
    }
}
