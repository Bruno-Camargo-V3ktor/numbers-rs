use dioxus::prelude::*;

use super::questions::QuestionsContainer;

#[component]
pub fn InfosContianer() -> Element {
    rsx! {
        div { class: "infos-container",
            h4 { class: "overline", "Online - Gratuito" }
            h1 { class: "header", "Sorteador de números" }
            QuestionsContainer {}
        }
    }
}
