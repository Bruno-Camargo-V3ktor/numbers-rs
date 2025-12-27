use crate::pages::MainPage;
use dioxus::prelude::*;

mod components;
mod pages;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        MainPage {}
    }
}
