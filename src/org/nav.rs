use dioxus::prelude::*;

use crate::org;

#[component]
pub fn Nav() -> Element {
    rsx! {
        div {
            class: "underline grid grid-cols-4 grid-rows-1 gap-5 text-center bg-gray-200",
            Link { 
                to: org::pages::Routes::Todo {}, "Todo",
            },
            Link { 
                to: org::pages::Routes::Goals {}, "Goals",
            },
            Link { 
                to: org::pages::Routes::Calendar {}, "Calendar",
            }
            Link { 
                to: org::pages::Routes::Timeline {}, "Timeline",
            }
        }
    }
}
