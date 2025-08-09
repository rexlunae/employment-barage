use dioxus::prelude::*;
use dioxus_bootstrap::*;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        nav {
            class: "navbar navbar-expand-lg navbar-dark bg-dark",
            id: "navbar",
            Container {
                {children}
            }
        }
    }
}