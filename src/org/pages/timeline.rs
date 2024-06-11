use dioxus::prelude::*;

use crate::org;

#[component]
pub fn Timeline() -> Element {
    rsx! {
        org::Page { name: "Timeline" }
    }
}
