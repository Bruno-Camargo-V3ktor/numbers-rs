use super::{
    form::FormContianer, infos::InfosContianer, questions::QuestionsContainer,
    result::ResultContainer,
};
use dioxus::prelude::*;

#[derive(Default, Clone)]
pub struct StateNumbers {
    pub numbers: Option<Vec<u16>>,
    pub counter: usize,
}

#[component]
pub fn ContentContainer() -> Element {
    let state = use_signal(|| StateNumbers::default());

    rsx! {
        div { class: "content-container",
            InfosContianer {}
            if state().numbers.is_none() {
                FormContianer { state }
            } else {
                ResultContainer { state }
            }
        }
        footer { class: "footer-mobile", style: "display: none", QuestionsContainer {} }
    }
}
