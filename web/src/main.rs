use dioxus::prelude::*;
use ui::App;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(|| rsx! {
        App {
            favicon: FAVICON,
            include_bootstrap: true,
            include_fontawesome: true,
            include_theme: true,
        }
    });
}

