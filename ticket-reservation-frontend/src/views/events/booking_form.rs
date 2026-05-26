use dioxus::prelude::*;

use crate::api;
use crate::models::{BookingRequest, BookingResponse};
use crate::AuthState;

const BOOKING_FORM_CSS: Asset = asset!("/assets/styling/events/booking_form.css");

#[component]
pub fn BookingForm(
    event_id: u64,
    available_capacity: u64,
    on_success: EventHandler<BookingResponse>,
    on_error: EventHandler<String>,
) -> Element {
    let auth_state = use_context::<AuthState>();

    let mut ticket_count = use_signal(|| "1".to_string());
    let mut booking_loading = use_signal(|| false);

    let is_sold_out = available_capacity == 0;

    rsx! {
        document::Link { rel: "stylesheet", href: BOOKING_FORM_CSS }

        div {
            class: "booking-form",

            label {
                class: "booking-form__label",
                "Tickets"
            }

            div {
                class: "booking-form__controls",

                input {
                    class: "booking-form__input",
                    r#type: "number",
                    min: "1",
                    max: "{available_capacity}",
                    value: "{ticket_count()}",
                    disabled: booking_loading() || is_sold_out,
                    oninput: move |event| {
                        ticket_count.set(event.value());
                    }
                }

                button {
                    class: "booking-form__button",
                    disabled: booking_loading() || is_sold_out,
                    onclick: move |_| {
                        let user_id_text = (auth_state.user_id)();
                        let tickets_text = ticket_count();

                        spawn(async move {
                            booking_loading.set(true);

                            let Some(user_id) = user_id_text else {
                                on_error.call("No se encontró el usuario autenticado.".to_string());
                                booking_loading.set(false);
                                return;
                            };

                            if user_id.trim().is_empty() {
                                on_error.call("El id del usuario autenticado está vacío.".to_string());
                                booking_loading.set(false);
                                return;
                            }

                            let ticket_count = match tickets_text.parse::<u64>() {
                                Ok(value) if value > 0 => value,
                                _ => {
                                    on_error.call("La cantidad de tickets debe ser mayor a cero.".to_string());
                                    booking_loading.set(false);
                                    return;
                                }
                            };

                            if ticket_count > available_capacity {
                                on_error.call(format!(
                                    "Solo quedan {} tickets disponibles para este evento.",
                                    available_capacity
                                ));
                                booking_loading.set(false);
                                return;
                            }

                            let request = BookingRequest {
                                user_id,
                                event_id,
                                ticket_count,
                            };

                            match api::create_booking(request).await {
                                Ok(response) => on_success.call(response),
                                Err(message) => on_error.call(message),
                            }

                            booking_loading.set(false);
                        });
                    },

                    if booking_loading() {
                        "Reservando..."
                    } else if is_sold_out {
                        "Agotado"
                    } else {
                        "Reservar"
                    }
                }
            }
        }
    }
}
