use dioxus::prelude::*;

const ERROR_STATE_CSS: Asset = asset!("/assets/styling/error-state.css");

#[component]
pub fn ErrorState(
    message: String,
    #[props(default = "Ocurrió un problema".to_string())] title: String,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ERROR_STATE_CSS }

        div {
            class: "error-state",
            role: "alert",

            div {
                class: "error-state__icon",
                aria_hidden: "true",
                "!"
            }

            div {
                class: "error-state__content",

                h3 {
                    "{title}"
                }

                p {
                    "{message}"
                }
            }
        }
    }
}
