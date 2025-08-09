use dioxus::prelude::*;
use ui::views::{Dashboard, Profile, Resumes, Jobs, Applications};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/profile")]
    Profile {},
    #[route("/resumes")]
    Resumes {},
    #[route("/jobs")]
    Jobs {},
    #[route("/applications")]
    Applications {},
}

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

