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
    let mut msg_error = use_signal(|| Option::<String>::None);

    let on_submit_form = move |evt: Event<MouseData>| {
        evt.prevent_default();

        let quantity = quantity_input();
        let from = from_input();
        let at = at_input();
        let repetions = repetions();

        if quantity == 0 {
            msg_error.set(Some(String::from(
                "A quantidade minima de números tem que ser maior que 0",
            )));
            return;
        }

        if (at <= from) || (repetions && (at - from) < quantity) {
            msg_error.set(Some(String::from("Intervalo de valores invalido")));
            return;
        }

        let nums = rand_rang(from, at, quantity as usize, !repetions);
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
                        msg_error: msg_error.clone(),
                    }

                    InputNumber {
                        title: "de",
                        default: from_input(),
                        min: 0,
                        max: 1000,
                        name: "from",
                        signal: from_input.clone(),
                        msg_error: msg_error.clone(),

                    }

                    InputNumber {
                        title: "Ate",
                        default: at_input(),
                        min: 1,
                        max: 1000,
                        name: "at",
                        signal: at_input.clone(),
                        msg_error: msg_error.clone(),
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

                if let Some(msg) = msg_error() {
                    p { class: "form-error", "{msg}" }
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
    pub msg_error: Signal<Option<String>>,
}

#[component]
fn InputNumber(props: InputNumberProps) -> Element {
    let mut signal = props.signal.clone();
    let mut msg_error = props.msg_error.clone();

    let on_changed = move |evt: Event<FormData>| {
        signal.set(evt.value().parse::<u16>().unwrap_or(0));
        msg_error.set(None);
    };

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

            oninput: on_changed,
        }
    }
}
