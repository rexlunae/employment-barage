use dioxus::prelude::*;
use dioxus_bootstrap::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        Container {
            class: "text-center py-5",
            id: "hero",
            Row {
                Col { lg: 8, offset_lg: 2,
                    img { 
                        src: HEADER_SVG, 
                        id: "header",
                        class: "img-fluid mb-4"
                    }
                    div { id: "links", class: "d-flex flex-wrap justify-content-center gap-3",
                        a { 
                            class: "btn btn-primary",
                            href: "https://dioxuslabs.com/learn/0.6/", 
                            target: "_blank",
                            "📚 Learn Dioxus" 
                        }
                        a { 
                            class: "btn btn-secondary",
                            href: "https://dioxuslabs.com/awesome", 
                            target: "_blank",
                            "🚀 Awesome Dioxus" 
                        }
                        a { 
                            class: "btn btn-success",
                            href: "https://github.com/dioxus-community/", 
                            target: "_blank",
                            "📡 Community Libraries" 
                        }
                        a { 
                            class: "btn btn-info",
                            href: "https://github.com/DioxusLabs/sdk", 
                            target: "_blank",
                            "⚙️ Dioxus Development Kit" 
                        }
                        a { 
                            class: "btn btn-warning",
                            href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", 
                            target: "_blank",
                            "💫 VSCode Extension" 
                        }
                        a { 
                            class: "btn btn-danger",
                            href: "https://discord.gg/XgGxMSkvUM", 
                            target: "_blank",
                            "👋 Community Discord" 
                        }
                    }
                }
            }
        }
    }
}