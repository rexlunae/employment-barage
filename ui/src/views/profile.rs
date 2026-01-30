//! Profile view - displays the profile management UI

use dioxus::prelude::*;
use crate::SharedNavigation;

/// Profile page view
#[component]
pub fn Profile() -> Element {
    rsx! { 
        document::Title { "Profile Manager - Employment Barage | Build Your Professional Profile" }
        div { class: "min-vh-100 bg-light",
            SharedNavigation {}
            
            main { class: "py-4",
                crate::ProfileManager { profile: use_signal(|| None) }
            }
        }
    }
}
