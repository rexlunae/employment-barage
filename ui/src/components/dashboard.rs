use dioxus::prelude::*;
use dioxus_bootstrap::*;
use api::{Profile, ParsedResume};
use crate::components::upload::ResumeUpload;

#[component]
pub fn Dashboard() -> Element {
    let mut current_view = use_signal(|| DashboardView::Welcome);
    let mut user_profile = use_signal(|| None::<Profile>);

    let handle_resume_upload = move |parsed: ParsedResume| {
        user_profile.set(Some(parsed.profile));
        current_view.set(DashboardView::Profile);
    };

    rsx! {
        div { class: "min-vh-100",
            // Navigation Bar
            nav { class: "navbar navbar-expand-lg shadow-sm",
                Container {
                    a { class: "navbar-brand fw-bold", href: "/",
                        i { class: "fas fa-briefcase me-2" }
                        "Employment Barage"
                    }
                    
                    div { class: "navbar-nav ms-auto",
                        button {
                            class: "nav-link btn btn-link border-0",
                            onclick: move |_| current_view.set(DashboardView::Welcome),
                            i { class: "fas fa-home me-1" }
                            "Home"
                        }
                        button {
                            class: "nav-link btn btn-link border-0",
                            onclick: move |_| current_view.set(DashboardView::Profile),
                            i { class: "fas fa-user me-1" }
                            "Profile"
                        }
                        button {
                            class: "nav-link btn btn-link border-0",
                            onclick: move |_| current_view.set(DashboardView::Resumes),
                            i { class: "fas fa-file-alt me-1" }
                            "Resumes"
                        }
                        button {
                            class: "nav-link btn btn-link border-0",
                            onclick: move |_| current_view.set(DashboardView::Jobs),
                            i { class: "fas fa-search me-1" }
                            "Jobs"
                        }
                        button {
                            class: "nav-link btn btn-link border-0",
                            onclick: move |_| current_view.set(DashboardView::Applications),
                            i { class: "fas fa-paper-plane me-1" }
                            "Applications"
                        }
                        Button { variant: ButtonVariant::Secondary, class: "ms-2",
                            i { class: "fas fa-adjust" }
                        }
                    }
                }
            }
            
            // Main Content
            main { class: "py-4",
                match current_view() {
                    DashboardView::Welcome => rsx! { WelcomeView { on_upload: handle_resume_upload } },
                    DashboardView::Profile => rsx! { ProfileView { profile: user_profile } },
                    DashboardView::Resumes => rsx! { ResumesView {} },
                    DashboardView::Jobs => rsx! { JobsView {} },
                    DashboardView::Applications => rsx! { ApplicationsView {} },
                }
            }
        }
    }
}

#[component]
fn WelcomeView(on_upload: EventHandler<ParsedResume>) -> Element {
    rsx! {
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
            ResumeUpload { on_upload }
        }
    }
}

#[component]
fn ProfileView(profile: Signal<Option<Profile>>) -> Element {
    rsx! {
        Container {
            Row {
                Col {
                    h2 { "Profile Management" }
                    if let Some(p) = profile() {
                        Card {
                            CardBody {
                                h5 { "Welcome!" }
                                p { "Email: {p.email}" }
                                if let Some(phone) = &p.phone {
                                    p { "Phone: {phone}" }
                                }
                                if let Some(location) = &p.location {
                                    p { "Location: {location}" }
                                }
                            }
                        }
                    } else {
                        div { class: "text-center text-muted py-5",
                            i { class: "fas fa-user fa-4x mb-3" }
                            h4 { "No profile data yet" }
                            p { "Upload your resume to get started!" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ResumesView() -> Element {
    rsx! {
        Container {
            Row {
                Col {
                    div { class: "d-flex justify-content-between align-items-center mb-4",
                        h2 { "My Resumes" }
                        Button { variant: ButtonVariant::Primary,
                            i { class: "fas fa-plus me-2" }
                            "Create New Resume"
                        }
                    }
                    
                    div { class: "text-center text-muted py-5",
                        i { class: "fas fa-file-alt fa-4x mb-3" }
                        h4 { "No resumes created yet" }
                        p { "Create your first resume to get started!" }
                    }
                }
            }
        }
    }
}

#[component]
fn JobsView() -> Element {
    rsx! {
        Container {
            Row {
                Col {
                    div { class: "d-flex justify-content-between align-items-center mb-4",
                        h2 { "Job Search" }
                        Button { variant: ButtonVariant::Primary,
                            i { class: "fas fa-search me-2" }
                            "New Search"
                        }
                    }
                    
                    Card {
                        CardBody {
                            div { class: "text-center text-muted py-5",
                                i { class: "fas fa-search fa-4x mb-3" }
                                h4 { "Search for opportunities" }
                                p { "Start searching for jobs across multiple platforms!" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ApplicationsView() -> Element {
    rsx! {
        Container {
            Row {
                Col {
                    div { class: "d-flex justify-content-between align-items-center mb-4",
                        h2 { "My Applications" }
                        ButtonGroup {
                            Button { variant: ButtonVariant::Primary, size: Size::Small, "All" }
                            Button { variant: ButtonVariant::Success, size: Size::Small, "Applied" }
                            Button { variant: ButtonVariant::Warning, size: Size::Small, "Interviewing" }
                            Button { variant: ButtonVariant::Info, size: Size::Small, "Offered" }
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

#[derive(Clone, PartialEq)]
enum DashboardView {
    Welcome,
    Profile,
    Resumes,
    Jobs,
    Applications,
}