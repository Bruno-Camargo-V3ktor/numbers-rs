use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub line: Option<String>,
    pub name: String,
    pub class: Option<String>,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let line = if let Some(line) = &props.line {
        format!("ph-{line}")
    } else {
        String::from("ph")
    };

    let classes = props.class.unwrap_or(String::new());

    rsx! {
        i { class: "icon {line} ph-{props.name} {classes}" }
    }
}
