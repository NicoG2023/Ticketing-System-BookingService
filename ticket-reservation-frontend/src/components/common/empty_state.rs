use dioxus::prelude::*;

const EMPTY_STATE_CSS: Asset = asset!("/assets/styling/empty-state.css");

#[component]
pub fn EmptyState(title: String, message: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: EMPTY_STATE_CSS }

        div {
            class: "empty-state",

            div {
                class: "empty-state__icon",
                aria_hidden: "true",
                "∅"
            }

            h3 {
                "{title}"
            }

            p {
                "{message}"
            }
        }
    }
}
