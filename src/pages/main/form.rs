use crate::{pages::main::content::StateNumbers, utils::random::rand_rang};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FormContianerProps {
    pub state: Signal<StateNumbers>,
}

#[component]
pub fn FormContianer(props: FormContianerProps) -> Element {
    let quantity_input = use_signal(|| 2);
    let from_input = use_signal(|| 1);
    let at_input = use_signal(|| 100);
    let mut repetions = use_signal(|| true);
    let mut state = props.state.clone();

    let on_submit_form = move |evt: Event<MouseData>| {
        evt.prevent_default();
        let nums = rand_rang(
            from_input(),
            at_input(),
            quantity_input() as usize,
            !repetions(),
        );

        let old_state = state().clone();
        state.set(StateNumbers {
            numbers: Some(nums),
            counter: old_state.counter + 1,
        });
    };

    rsx! {
        div { class: "form-container",
            h3 { class: "form-title", "Quero sortear:" }
            p { class: "form-description",
                r#"defina o intervalo e a quantidade de números, clique em "Sortear" e veja os resultados na tela. É rápido e fácil!"#
            }
            form {
                onsubmit: |evt| {
                    evt.prevent_default();
                },
                div { class: "numbers-inputs",
                    InputNumber {
                        title: "números",
                        default: quantity_input(),
                        min: 1,
                        max: 100,
                        name: "quantity",
                        signal: quantity_input.clone(),
                    }

                    InputNumber {
                        title: "de",
                        default: from_input(),
                        min: 0,
                        max: 1000,
                        name: "from",
                        signal: from_input.clone(),
                    }

                    InputNumber {
                        title: "Ate",
                        default: at_input(),
                        min: 1,
                        max: 1000,
                        name: "at",
                        signal: at_input.clone(),
                    }
                }
                input {
                    class: "checkbox-repetion",
                    r#type: "checkbox",
                    checked: "true",
                    oninput: move |e| {
                        repetions.set(e.value().parse::<bool>().unwrap());
                    },
                }

                button { class: "btn-random", onclick: on_submit_form,
                    div {
                        "Sortear"
                        i { class: "ph ph-arrow-right" }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct InputNumberProps {
    pub title: String,
    pub name: String,
    pub default: u16,
    pub min: u16,
    pub max: u16,
    pub signal: Signal<u16>,
}

#[component]
fn InputNumber(props: InputNumberProps) -> Element {
    let mut signal = props.signal.clone();

    rsx! {
        label { class: "number-label", r#for: "{props.name}", "{props.title}" }
        input {
            class: "number-input",
            name: "{props.name}",
            id: "{props.name}",
            r#type: "number",
            min: "{props.min}",
            max: "{props.max}",
            value: "{props.default}",

            oninput: move |evt| signal.set(evt.value().parse::<u16>().unwrap()),
        }
    }
}
