use dioxus::prelude::*;
use crate::{Container, Button, ButtonVariant};

#[component]
pub fn SharedNavigation() -> Element {
    rsx! {
        nav { class: "navbar navbar-expand-lg navbar-light shadow-sm",
            Container {
                a { href: "/", class: "navbar-brand fw-bold",
                    i { class: "fas fa-briefcase me-2" }
                    "Employment Barage"
                }
                
                div { class: "navbar-nav ms-auto",
                    a {
                        href: "/",
                        class: "nav-link btn btn-link border-0",
                        i { class: "fas fa-home me-1" }
                        "Dashboard"
                    }
                    a {
                        href: "/profile",
                        class: "nav-link btn btn-link border-0",
                        i { class: "fas fa-user me-1" }
                        "Profile"
                    }
                    a {
                        href: "/resumes",
                        class: "nav-link btn btn-link border-0",
                        i { class: "fas fa-file-alt me-1" }
                        "Resumes"
                    }
                    a {
                        href: "/jobs",
                        class: "nav-link btn btn-link border-0",
                        i { class: "fas fa-search me-1" }
                        "Jobs"
                    }
                    a {
                        href: "/applications",
                        class: "nav-link btn btn-link border-0",
                        i { class: "fas fa-paper-plane me-1" }
                        "Applications"
                    }
                    Button { variant: ButtonVariant::Secondary, class: "ms-2",
                        i { class: "fas fa-adjust" }
                    }
                }
            }
        }
    }
}