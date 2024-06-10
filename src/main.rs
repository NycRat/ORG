#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
mod org;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(
            r#"
            <link rel="stylesheet" href="tailwind.css">
            <link rel="stylesheet" href="main.css">
            "#.to_string()
            );
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(12342i32));
    rsx! {
        div {
            class: "h-screen",
            Router::<org::pages::Routes> {},
        }
    }
}
