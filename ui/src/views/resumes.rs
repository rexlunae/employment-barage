use dioxus::prelude::*;
use crate::SharedNavigation;

#[component]
pub fn Resumes() -> Element {
    rsx! { 
        document::Title { "Resume Builder - Employment Barage | Create ATS-Friendly Resumes" }
        div { class: "min-vh-100",
            SharedNavigation {}
            
            main { class: "py-4",
                // Resume builder component
                crate::ResumeBuilder {}
            }
        }
    }
}