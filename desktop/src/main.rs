use dioxus::prelude::*;
use ui::Route;


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        Router::<Route> {}
    }
}

