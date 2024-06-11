use dioxus::prelude::*;

use crate::org;

#[component]
pub fn Goals() -> Element {
    let mut goals_list_actual = use_context::<Signal<Vec<org::state::Goal>>>();
    let mut goal_create_open = use_signal(|| false);

    rsx! {
        org::Page { name: "Goals",
            ul {
                for goal in goals_list_actual() {
                    li { key: "{goal.id}", {std::format!("- {:?}", goal.name)} }
                }
                button {
                    onclick: move |_event| {
                        goal_create_open.set(!goal_create_open());
                    },
                    "New Goal"
                }
            }
            if goal_create_open() {
                org::views::GoalCreateForm { open: goal_create_open }
            }
        }
    }
}
