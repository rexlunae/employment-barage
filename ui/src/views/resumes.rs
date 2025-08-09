use dioxus::prelude::*;

#[component]
pub fn Resumes(navigation: Element) -> Element {
    rsx! { 
        div { class: "min-vh-100",
            {navigation}
            
            main { class: "py-4",
                // Resume builder component
                crate::ResumeBuilder {}
            }
        }
    }
}