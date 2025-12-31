use dioxus::prelude::*;

use crate::components::Icon;

#[component]
pub fn QuestionsContainer() -> Element {
    rsx! {
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
            Icon { name: "seal-question", class: "question-icon" }
            div { class: "question-texts",
                h5 { class: "question-title", "{props.question}" }
                p { class: "question-description", "{props.description}" }
            }
        }
    }
}
