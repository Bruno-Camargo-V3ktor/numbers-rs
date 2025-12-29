use crate::pages::main::content::StateNumbers;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ResultContainerProps {
    pub state: Signal<StateNumbers>,
}

#[component]
pub fn ResultContainer(props: ResultContainerProps) -> Element {
    let mut state = props.state.clone();

    let on_click_reset = move |evt: Event<MouseData>| {
        evt.prevent_default();
        let old_state = state().clone();
        state.set(StateNumbers {
            numbers: None,
            ..old_state
        });
    };

    rsx! {
        div { class: "result-container",
            h3 { class: "result-title", "Resultado do sorteio" }
            p { class: "result-count", "{state().counter}º resultado" }
            div { class: "result-numbers",
                for n in &state().numbers.unwrap() {
                    ResultNumber { number: *n }
                }
            }
            button { class: "result-reset-btn", onclick: on_click_reset,
                div {
                    span { "Sortear novamente" }
                    i { class: "ph ph-clock-counter-clockwise" }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ResultNumberProps {
    pub number: u16,
}
#[component]
pub fn ResultNumber(props: ResultNumberProps) -> Element {
    rsx! {
        div { class: "number-container",
            span { class: "number", "{props.number}" }
        }
    }
}
