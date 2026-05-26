use dioxus::prelude::*;

use crate::api;
use crate::models::CreateVenueRequest;

#[component]
pub fn CreateVenueForm(
    action_loading: Signal<bool>,
    on_success: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let mut venue_name = use_signal(|| String::new());
    let mut venue_address = use_signal(|| String::new());
    let mut venue_capacity = use_signal(|| "1000".to_string());

    rsx! {
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
                        let on_success = on_success;
                        let on_error = on_error;

                        spawn(async move {
                            action_loading.set(true);

                            if name.trim().is_empty() {
                                on_error.call(
                                    "El nombre de la sede es obligatorio.".to_string()
                                );
                                action_loading.set(false);
                                return;
                            }

                            if address.trim().is_empty() {
                                on_error.call(
                                    "La dirección de la sede es obligatoria.".to_string()
                                );
                                action_loading.set(false);
                                return;
                            }

                            let total_capacity = match capacity_text.parse::<u64>() {
                                Ok(value) if value > 0 => value,
                                _ => {
                                    on_error.call(
                                        "La capacidad debe ser mayor a cero.".to_string()
                                    );
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
                                    on_success.call(format!(
                                        "Sede creada: {}",
                                        created_venue.venue_name
                                    ));

                                    venue_name.set(String::new());
                                    venue_address.set(String::new());
                                    venue_capacity.set("1000".to_string());
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
                    } else {
                        "Crear sede"
                    }
                }
            }
        }
    }
}
