use dioxus::prelude::*;
use dioxus_bootstrap::GlobalTheme;
use ui::Route;

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

