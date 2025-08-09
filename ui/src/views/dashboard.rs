use dioxus::prelude::*;
use crate::{Container, Row, Col, Card, CardBody, Button, ButtonVariant, Size};
use api;

#[component]
pub fn Dashboard(navigation: Element) -> Element {
    let mut user_profile = use_signal(|| None);

    let handle_resume_upload = move |parsed: api::ParsedResume| {
        user_profile.set(Some(parsed.profile));
    };

    rsx! { 
        div { class: "min-vh-100",
            {navigation}
            
            main { class: "py-4",
                Container {
                    // Hero Section
                    Row { class: "mb-5",
                        Col { lg: 8, offset_lg: 2, class: "text-center",
                            div { class: "py-5",
                                h1 { class: "display-4 fw-bold text-primary mb-4",
                                    "Transform Your Career Journey"
                                }
                                p { class: "lead text-muted mb-4",
                                    "Upload your resume, build stunning profiles, and automate your job applications with AI-powered insights."
                                }
                                div { class: "d-flex justify-content-center gap-3",
                                    Button { variant: ButtonVariant::Primary, size: Size::Large, "Get Started" }
                                    Button { variant: ButtonVariant::Primary, size: Size::Large, "Learn More" }
                                }
                            }
                        }
                    }
                    
                    // Features Section
                    Row { class: "mb-5",
                        Col { lg: 4, class: "mb-4",
                            Card { class: "h-100 border-0 shadow-sm",
                                CardBody { class: "text-center p-4",
                                    div { class: "mb-3",
                                        i { class: "fas fa-upload fa-3x text-primary" }
                                    }
                                    h4 { "Smart Resume Parsing" }
                                    p { class: "text-muted",
                                        "Upload your existing resume and let our AI extract and organize your information automatically."
                                    }
                                }
                            }
                        }
                        Col { lg: 4, class: "mb-4",
                            Card { class: "h-100 border-0 shadow-sm",
                                CardBody { class: "text-center p-4",
                                    div { class: "mb-3",
                                        i { class: "fas fa-magic fa-3x text-success" }
                                    }
                                    h4 { "Resume Generation" }
                                    p { class: "text-muted",
                                        "Create beautiful, ATS-friendly resumes with multiple templates and customization options."
                                    }
                                }
                            }
                        }
                        Col { lg: 4, class: "mb-4",
                            Card { class: "h-100 border-0 shadow-sm",
                                CardBody { class: "text-center p-4",
                                    div { class: "mb-3",
                                        i { class: "fas fa-rocket fa-3x text-warning" }
                                    }
                                    h4 { "Job Automation" }
                                    p { class: "text-muted",
                                        "Automate job applications with personalized cover letters and streamlined submissions."
                                    }
                                }
                            }
                        }
                    }
                    
                    // Upload Section
                    crate::ResumeUpload { on_upload: handle_resume_upload }
                }
            }
        }
    }
}