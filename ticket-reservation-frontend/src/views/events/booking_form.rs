use dioxus::prelude::*;

use crate::api;
use crate::models::{BookingRequest, BookingResponse};

const BOOKING_FORM_CSS: Asset = asset!("/assets/styling/events/booking_form.css");

#[component]
pub fn BookingForm(
    event_id: u64,
    event_name: String,
    ticket_price: f64,
    available_capacity: u64,
    on_success: EventHandler<BookingResponse>,
    on_error: EventHandler<String>,
) -> Element {
    let mut ticket_count = use_signal(|| "1".to_string());
    let mut booking_loading = use_signal(|| false);
    let mut show_confirmation = use_signal(|| false);
    let mut pending_ticket_count = use_signal(|| None::<u64>);

    let is_sold_out = available_capacity == 0;

    let selected_tickets = pending_ticket_count().unwrap_or(1);
    let total_price = selected_tickets as f64 * ticket_price;
    let formatted_unit_price = format!("{:.2}", ticket_price);
    let formatted_total_price = format!("{:.2}", total_price);

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
                        let tickets_text = ticket_count();

                        let parsed_ticket_count = match tickets_text.parse::<u64>() {
                            Ok(value) if value > 0 => value,
                            _ => {
                                on_error.call(
                                    "La cantidad de tickets debe ser mayor a cero.".to_string()
                                );
                                return;
                            }
                        };

                        if parsed_ticket_count > available_capacity {
                            on_error.call(format!(
                                "Solo quedan {} tickets disponibles para este evento.",
                                available_capacity
                            ));
                            return;
                        }

                        pending_ticket_count.set(Some(parsed_ticket_count));
                        show_confirmation.set(true);
                    },

                    if is_sold_out {
                        "Agotado"
                    } else {
                        "Reservar"
                    }
                }
            }
        }

        if show_confirmation() {
            div {
                class: "booking-modal__backdrop",

                div {
                    class: "booking-modal",
                    role: "dialog",

                    div {
                        class: "booking-modal__icon",
                        "🎟️"
                    }

                    div {
                        class: "booking-modal__content",

                        p {
                            class: "booking-modal__eyebrow",
                            "Confirmar reserva"
                        }

                        h2 {
                            class: "booking-modal__title",
                            "¿Deseas reservar estos tickets?"
                        }

                        p {
                            class: "booking-modal__description",
                            "Revisa los detalles antes de confirmar tu reserva."
                        }

                        div {
                            class: "booking-modal__summary",

                            div {
                                class: "booking-modal__row",
                                span { "Evento" }
                                strong { "{event_name}" }
                            }

                            div {
                                class: "booking-modal__row",
                                span { "Tickets" }
                                strong { "{selected_tickets}" }
                            }

                            div {
                                class: "booking-modal__row",
                                span { "Precio unitario" }
                                strong { "${formatted_unit_price}" }
                            }

                            div {
                                class: "booking-modal__row booking-modal__row--total",
                                span { "Total" }
                                strong { "${formatted_total_price}" }
                            }
                        }
                    }

                    div {
                        class: "booking-modal__actions",

                        button {
                            class: "booking-modal__button booking-modal__button--secondary",
                            disabled: booking_loading(),
                            onclick: move |_| {
                                show_confirmation.set(false);
                                pending_ticket_count.set(None);
                            },
                            "Cancelar"
                        }

                        button {
                            class: "booking-modal__button booking-modal__button--primary",
                            disabled: booking_loading(),
                            onclick: move |_| {
                                let ticket_count_to_book = match pending_ticket_count() {
                                    Some(value) => value,
                                    None => {
                                        show_confirmation.set(false);
                                        return;
                                    }
                                };

                                spawn(async move {
                                    booking_loading.set(true);

                                    let request = BookingRequest {
                                        event_id,
                                        ticket_count: ticket_count_to_book,
                                    };

                                    match api::create_booking(request).await {
                                        Ok(response) => {
                                            on_success.call(response);
                                            ticket_count.set("1".to_string());
                                            pending_ticket_count.set(None);
                                            show_confirmation.set(false);
                                        }
                                        Err(message) => {
                                            on_error.call(message);
                                        }
                                    }

                                    booking_loading.set(false);
                                });
                            },

                            if booking_loading() {
                                "Confirmando..."
                            } else {
                                "Confirmar reserva"
                            }
                        }
                    }
                }
            }
        }
    }
}
