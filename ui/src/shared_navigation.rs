use dioxus::prelude::*;
use crate::{Container, Button, ButtonVariant};

#[component]
pub fn SharedNavigation(children: Element) -> Element {
    rsx! {
        nav { class: "navbar navbar-expand-lg navbar-light shadow-sm",
            Container {
                div { class: "navbar-brand fw-bold",
                    i { class: "fas fa-briefcase me-2" }
                    "Employment Barage"
                }
                
                div { class: "navbar-nav ms-auto",
                    {children}
                    Button { variant: ButtonVariant::Secondary, class: "ms-2",
                        i { class: "fas fa-adjust" }
                    }
                }
            }
        }
    }
}