use dioxus::prelude::*;

use content::ContentContainer;

mod content;
mod form;
mod infos;
mod questions;
mod result;

const BG_IMAGE: Asset = asset!("/assets/bg.png");

#[component]
pub fn MainPage() -> Element {
    rsx! {
        div { class: "main-container", background_image: "url({BG_IMAGE})", ContentContainer {} }
    }
}
