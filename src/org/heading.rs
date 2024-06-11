use dioxus::prelude::*;

#[component]
pub fn Heading(name: String) -> Element {
    rsx! {
        h1 { class: "text-4xl font-bold", "{name}" }
    }
}
