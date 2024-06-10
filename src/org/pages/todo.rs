use dioxus::prelude::*;

use crate::org;

#[component]
pub fn Todo() -> Element {
    rsx! {
        org::Page {
            name: "Todo",
        }
    }
}
