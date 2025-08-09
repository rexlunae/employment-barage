use dioxus::prelude::*;
use crate::SharedNavigation;

#[component]
pub fn Resumes() -> Element {
    rsx! { 
        div { class: "min-vh-100",
            SharedNavigation {}
            
            main { class: "py-4",
                // Resume builder component
                crate::ResumeBuilder {}
            }
        }
    }
}