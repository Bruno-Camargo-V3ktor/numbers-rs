use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/icon.svg");

#[component]
pub fn TitleComponent() -> Element {
    rsx! {
        div { class: "title-container",
            img { src: FAVICON }
            h1 { "NUMBERS" }
        }
    }
}
