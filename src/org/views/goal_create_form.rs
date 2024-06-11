use dioxus::prelude::*;

use crate::org;

#[component]
pub fn GoalCreateForm(mut open: Signal<bool>) -> Element {
    let mut goals_list = use_context::<Signal<Vec<org::state::Goal>>>();
    let mut name = use_signal(|| "".to_string());

    rsx! {
        div {
            div { class: "bg-gray-800 absolute left-0 right-0 top-0 bottom-0 opacity-50" }
            div { class: "absolute z-10 bg-gray-300",
                form {
                    onsubmit: move |event| {
                        println!("SUBMITTED: {event:?}");
                        goals_list
                            .push(org::state::Goal {
                                id: cuid2::create_id(),
                                name: name(),
                                tasks: vec![],
                            });
                        open.set(false);
                    },
                    label { "Name: " }
                    input {
                        oninput: move |event| {
                            name.set(event.data().value());
                            println!("{}", event.data().value());
                        }
                    }
                }
            }
        }
    }
}
