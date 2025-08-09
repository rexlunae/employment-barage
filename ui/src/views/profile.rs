use dioxus::prelude::*;
use crate::SharedNavigation;

#[component]
pub fn Profile() -> Element {
    rsx! { 
        document::Title { "Profile Manager - Employment Barage | Build Your Professional Profile" }
        div { class: "min-vh-100",
            SharedNavigation {}
            
            main { class: "py-4",
                // Profile manager component
                crate::ProfileManager { profile: use_signal(|| None) }
            }
        }
    }
}