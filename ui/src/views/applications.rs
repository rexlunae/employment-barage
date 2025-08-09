use dioxus::prelude::*;
use crate::{Container, Row, Col, Button, ButtonVariant, ButtonGroup, Size};

#[component]
pub fn Applications(navigation: Element) -> Element {
    rsx! { 
        div { class: "min-vh-100",
            {navigation}
            
            main { class: "py-4",
                Container {
                    Row {
                        Col {
                            div { class: "d-flex justify-content-between align-items-center mb-4",
                                h2 { "My Applications" }
                                ButtonGroup {
                                    Button { variant: ButtonVariant::Primary, size: Size::Small, "All" }
                                    Button { variant: ButtonVariant::Secondary, size: Size::Small, "Applied" }
                                    Button { variant: ButtonVariant::Secondary, size: Size::Small, "Interviewing" }
                                    Button { variant: ButtonVariant::Secondary, size: Size::Small, "Offered" }
                                }
                            }
                            
                            div { class: "text-center text-muted py-5",
                                i { class: "fas fa-paper-plane fa-4x mb-3" }
                                h4 { "No applications yet" }
                                p { "Your job applications will appear here once you start applying!" }
                            }
                        }
                    }
                }
            }
        }
    }
}