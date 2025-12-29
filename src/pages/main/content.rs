use crate::utils::random::rand_rang;
use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn ContentContainer() -> Element {
    let quantity_input = use_signal(|| 2);
    let from_input = use_signal(|| 1);
    let at_input = use_signal(|| 100);
    let mut repetions = use_signal(|| true);
    let mut numbers = use_signal(|| Option::<Vec<u16>>::None);

    println!("teste");
    let on_submit_form = move |evt: Event<MouseData>| {
        evt.prevent_default();
        let nums = rand_rang(
            from_input(),
            at_input(),
            quantity_input() as usize,
            repetions(),
        );
        numbers.set(Some(nums));
        tracing::info!("{:?}", numbers());
    };

    rsx! {
        div { class: "content-container",
            div { class: "infos-container",
                h4 { class: "overline", "Online - Gratuito" }
                h1 { class: "header", "Sorteador de números" }
                div { class: "questions",
                    Question {
                        question: "Como funciona o sorteador de números?",
                        description: "O sorteador utiliza um algoritmo de geração aleatória para criar números dentro do intervalo especificado pelo usuário.",
                    }

                    Question {
                        question: "Posso escolher o intervalo dos números?",
                        description: "Sim, você pode definir os valores mínimo e máximo para o intervalo dos números sorteados.",
                    }
                }
            }

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
}

#[derive(Props, PartialEq, Clone)]
struct QuestionProps {
    pub question: String,
    pub description: String,
}

#[component]
fn Question(props: QuestionProps) -> Element {
    rsx! {
        div { class: "question-container",
            i { class: "ph ph-seal-question" }
            div { class: "question-texts",
                h5 { class: "question-title", "{props.question}" }
                p { class: "question-description", "{props.description}" }
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
