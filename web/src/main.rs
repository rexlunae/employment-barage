use dioxus::prelude::*;
use dioxus_bootstrap::GlobalTheme;
use ui::{DashboardView, ProfileView, ResumesView, JobsView, ApplicationsView};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/profile")]
    Profile {},
    #[route("/resumes")]
    Resumes {},
    #[route("/jobs")]
    Jobs {},
    #[route("/applications")]
    Applications {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { 
            rel: "stylesheet", 
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" 
        }
        document::Link { 
            rel: "stylesheet", 
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css" 
        }
        
        // Global theme management - must come after stylesheets
        GlobalTheme {}

        Router::<Route> {}
    }
}

fn create_navigation() -> Element {
    rsx! {
        Link {
            to: Route::Dashboard {},
            class: "nav-link btn btn-link border-0",
            i { class: "fas fa-home me-1" }
            "Dashboard"
        }
        Link {
            to: Route::Profile {},
            class: "nav-link btn btn-link border-0",
            i { class: "fas fa-user me-1" }
            "Profile"
        }
        Link {
            to: Route::Resumes {},
            class: "nav-link btn btn-link border-0",
            i { class: "fas fa-file-alt me-1" }
            "Resumes"
        }
        Link {
            to: Route::Jobs {},
            class: "nav-link btn btn-link border-0",
            i { class: "fas fa-search me-1" }
            "Jobs"
        }
        Link {
            to: Route::Applications {},
            class: "nav-link btn btn-link border-0",
            i { class: "fas fa-paper-plane me-1" }
            "Applications"
        }
    }
}

#[component]
fn Dashboard() -> Element {
    let navigation = rsx! {
        ui::SharedNavigation { 
            children: create_navigation()
        }
    };
    rsx! { DashboardView { navigation: navigation } }
}

#[component] 
fn Profile() -> Element {
    let navigation = rsx! {
        ui::SharedNavigation { 
            children: create_navigation()
        }
    };
    rsx! { ProfileView { navigation: navigation } }
}

#[component]
fn Resumes() -> Element {
    let navigation = rsx! {
        ui::SharedNavigation { 
            children: create_navigation()
        }
    };
    rsx! { ResumesView { navigation: navigation } }
}

#[component]
fn Jobs() -> Element {
    let navigation = rsx! {
        ui::SharedNavigation { 
            children: create_navigation()
        }
    };
    rsx! { JobsView { navigation: navigation } }
}

#[component]
fn Applications() -> Element {
    let navigation = rsx! {
        ui::SharedNavigation { 
            children: create_navigation()
        }
    };
    rsx! { ApplicationsView { navigation: navigation } }
}
