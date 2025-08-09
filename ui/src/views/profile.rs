use dioxus::prelude::*;

#[component]
pub fn Profile(navigation: Element) -> Element {
    rsx! { 
        div { class: "min-vh-100",
            {navigation}
            
            main { class: "py-4",
                // Profile manager component
                crate::ProfileManager { profile: use_signal(|| None) }
            }
        }
    }
}