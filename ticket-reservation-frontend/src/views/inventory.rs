use dioxus::prelude::*;

use crate::api;
use crate::components::RequireAuth;
use crate::models::{CreateEventRequest, EventInventoryResponse, VenueInventoryResponse};

const INVENTORY_CSS: Asset = asset!("/assets/styling/inventory.css");

#[component]
pub fn Inventory() -> Element {
    rsx! {
        RequireAuth {
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

    let mut event_name = use_signal(|| String::new());
    let mut event_capacity = use_signal(|| "100".to_string());
    let mut event_price = use_signal(|| "150000".to_string());
    let mut selected_venue_id = use_signal(|| String::new());

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
                Ok(data) => {
                    if selected_venue_id().is_empty() {
                        if let Some(first_venue) = data.first() {
                            selected_venue_id.set(first_venue.venue_id.to_string());
                        }
                    }

                    venues.set(data);
                }
                Err(message) => error.set(Some(message)),
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

                            spawn(async move {
                                action_loading.set(true);
                                error.set(None);
                                success.set(None);

                                if name.trim().is_empty() {
                                    error.set(Some("El nombre del evento es obligatorio.".to_string()));
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

                                if venue_id_text.trim().is_empty() {
                                    error.set(Some("Debes seleccionar una sede.".to_string()));
                                    action_loading.set(false);
                                    return;
                                }

                                let venue_id = match venue_id_text.parse::<u64>() {
                                    Ok(value) if value > 0 => value,
                                    _ => {
                                        error.set(Some("La sede seleccionada no es válida.".to_string()));
                                        action_loading.set(false);
                                        return;
                                    }
                                };

                                let ticket_price = match price_text.parse::<f64>() {
                                    Ok(value) if value > 0.0 => value,
                                    _ => {
                                        error.set(Some("El precio debe ser mayor a cero.".to_string()));
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
                                        success.set(Some(format!(
                                            "Evento creado: {}",
                                            created_event.event
                                        )));

                                        event_name.set(String::new());
                                        event_capacity.set("100".to_string());
                                        event_price.set("150000".to_string());

                                        reload_inventory();
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
                        } else if venues().is_empty() {
                            "Sin sedes"
                        } else {
                            "Crear evento"
                        }
                    }
                }
            }

            section {
                class: "panel",

                div {
                    class: "panel-header",

                    div {
                        h2 { "Eventos registrados" }
                        p { "Administra la capacidad disponible de cada evento." }
                    }

                    span {
                        class: "count-badge",
                        "{events().len()} eventos"
                    }
                }

                if loading() {
                    p {
                        class: "muted",
                        "Cargando inventario..."
                    }
                } else if events().is_empty() {
                    div {
                        class: "empty-state",
                        h3 { "No hay eventos registrados" }
                        p { "Crea el primer evento usando el formulario superior." }
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
                class: "inventory-card__top",

                div {
                    h3 { "{event.event}" }
                    p { "ID {event.event_id}" }
                }

                span {
                    class: "capacity-badge",
                    "{event.capacity}"
                }
            }

            div {
                class: "inventory-card__details",

                p {
                    span { "Sede" }
                    strong { "{event.venue}" }
                }

                p {
                    span { "Precio" }
                    strong { "${event.ticket_price}" }
                }
            }

            div {
                class: "inventory-card__actions",

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
                        class: "ghost-button danger",
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
                            "..."
                        } else {
                            "Descontar"
                        }
                    }

                    button {
                        class: "ghost-button success",
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
                            "..."
                        } else {
                            "Liberar"
                        }
                    }
                }
            }
        }
    }
}
