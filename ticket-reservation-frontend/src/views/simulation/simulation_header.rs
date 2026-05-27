use dioxus::prelude::*;

const SIMULATION_HEADER_CSS: Asset = asset!("/assets/styling/simulation/simulation_header.css");

#[component]
pub fn SimulationHeader() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: SIMULATION_HEADER_CSS
        }

        section {
            class: "simulation-hero",

            p {
                class: "simulation-hero__eyebrow",
                "Simulación de errores y concurrencia"
            }

            h1 {
                "Lost Update"
            }

            p {
                class: "simulation-hero__description",
                "Observa cómo dos reservas concurrentes pueden leer la misma capacidad inicial y sobrescribir sus resultados cuando no se usa una transacción."
            }
        }
    }
}
