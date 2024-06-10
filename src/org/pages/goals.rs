use dioxus::prelude::*;

use crate::org;

#[component]
pub fn Goals() -> Element {
    let goals_list = vec!["haha", "other thing", "asf"];
    let test = use_context::<Signal<i32>>();

    rsx! {
        org::Page {
            name: "Goals",
            ul {
                for goal in goals_list {
                    li { {std::format!("- {}", goal)} }
                }
                button { 
                    onclick: move |_event| println!("asf"),
                    {std::format!("{}", test())},
                }
            }
        }
    }
}
