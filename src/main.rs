use crate::pages::MainPage;
use dioxus::prelude::*;

mod components;
mod pages;
mod utils;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Roboto+Flex:opsz,wght,XOPQ,XTRA,YOPQ,YTDE,YTFI,YTLC,YTUC@8..144,100..1000,96,468,79,-203,738,514,712&display=swap",
        }
        //
        document::Link {
            rel: "stylesheet",
            r#type: "text/css",
            href: "https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/regular/style.css",
        }
        document::Link {
            rel: "stylesheet",
            r#type: "text/css",
            href: "https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/fill/style.css",
        }
        //

        document::Link { rel: "stylesheet", href: MAIN_CSS }
        MainPage {}
    }
}
