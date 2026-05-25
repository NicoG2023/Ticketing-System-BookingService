use dioxus::prelude::*;

const LOADING_STATE_CSS: Asset = asset!("/assets/styling/loading-state.css");

#[component]
pub fn LoadingState(
    #[props(default = "Cargando datos...".to_string())] message: String,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOADING_STATE_CSS }

        div {
            class: "loading-state",
            role: "status",
            aria_live: "polite",

            div {
                class: "loading-spinner",
                aria_hidden: "true"
            }

            p {
                "{message}"
            }
        }
    }
}
