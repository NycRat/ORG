use dioxus::prelude::*;

use crate::org;

#[component]
pub fn Calendar() -> Element {
    rsx! {
        org::Page { name: "Calendar", "stuff herer" }
    }
}
