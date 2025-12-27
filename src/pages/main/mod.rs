use dioxus::prelude::*;

const BG_IMAGE: Asset = asset!("/assets/bg.png");

#[component]
pub fn MainPage() -> Element {
    rsx! {
        div { class: "main-container", background_image: "url({BG_IMAGE})" }
    }
}
