use dioxus::prelude::*;

use crate::org;

pub mod todo;
use todo::Todo;
pub mod goals;
use goals::Goals;
pub mod calendar;
use calendar::Calendar;
pub mod timeline;
use timeline::Timeline;

#[component]
pub fn Page(name: String, children: Element) -> Element {
    rsx! {
        div {
            div {
                class: "p-8",
                org::Heading {
                    name
                },
                {children},
            },
            div {
                class: "fixed bottom-0 left-0 right-0",
                org::Nav {},
            },
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Routes {
    #[route("/")]
    Todo {},
    #[route("/goals")]
    Goals {},
    #[route("/calendar")]
    Calendar {},
    #[route("/timeline")]
    Timeline {},
}
