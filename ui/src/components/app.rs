use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn App(
    #[props(optional)] favicon: Option<Asset>,
    #[props(optional)] include_bootstrap: Option<bool>,
    #[props(optional)] include_fontawesome: Option<bool>,
    #[props(optional)] include_theme: Option<bool>,
) -> Element {
    let include_bootstrap = include_bootstrap.unwrap_or(true);
    let include_fontawesome = include_fontawesome.unwrap_or(true);
    let include_theme = include_theme.unwrap_or(true);

    rsx! {
        // Favicon if provided
        if let Some(favicon_asset) = favicon {
            document::Link { rel: "icon", href: favicon_asset }
        }
        
        // Bootstrap CSS
        if include_bootstrap {
            document::Link { 
                rel: "stylesheet", 
                href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" 
            }
        }
        
        // Font Awesome
        if include_fontawesome {
            document::Link { 
                rel: "stylesheet", 
                href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css" 
            }
        }
        
        // Global theme management - must come after stylesheets
        if include_theme {
            crate::GlobalTheme {}
        }

        Router::<Route> {}
    }
}

// Simple App version for platforms that don't need all the extras
#[component]
pub fn SimpleApp() -> Element {
    rsx! {
        Router::<Route> {}
    }
}