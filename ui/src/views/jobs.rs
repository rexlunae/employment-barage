use dioxus::prelude::*;

#[component]
pub fn Jobs(navigation: Element) -> Element {
    rsx! { 
        div { class: "min-vh-100",
            {navigation}
            
            main { class: "py-4",
                // Job search component
                crate::JobSearch {}
            }
        }
    }
}