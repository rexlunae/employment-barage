use dioxus::prelude::*;
use crate::SharedNavigation;

#[component]
pub fn Jobs() -> Element {
    rsx! { 
        div { class: "min-vh-100",
            SharedNavigation {}
            
            main { class: "py-4",
                // Job search component
                crate::JobSearch {}
            }
        }
    }
}