#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
mod org;

fn main() {
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        LaunchBuilder::web().with_cfg(cfg).launch(App);
    }
    #[cfg(feature = "desktop")]
    {
        dioxus_logger::init(Level::INFO).expect("failed to init logger");

        let cfg = dioxus::desktop::Config::new().with_custom_head(
            r#"
                <link rel="stylesheet" href="tailwind.css">
                <link rel="stylesheet" href="main.css">
                "#
            .to_string(),
        );
        LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    }
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(vec![org::state::Goal {
            id: cuid2::create_id(),
            name: "make video".into(),
            tasks: vec![
                org::state::Task {
                    name: "idea".into(),
                },
                org::state::Task {
                    name: "code".into(),
                },
                org::state::Task {
                    name: "edit".into(),
                },
            ],
        }])
    });
    rsx! {
        div { class: "h-screen", Router::<org::pages::Routes> {} }
    }
}
