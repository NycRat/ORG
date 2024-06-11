use dioxus::prelude::*;

use crate::org;

#[component]
pub fn Todo() -> Element {
    let mut goals_list = use_context::<Signal<Vec<org::state::Goal>>>();

    rsx! {
        org::Page { name: "Todo",
            ul {
                for goal in goals_list() {
                    li { key: "{goal.name}", "{goal.name}" }
                }
            }
        }
    }
}
