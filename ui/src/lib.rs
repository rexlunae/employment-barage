//! This crate contains all shared UI for the workspace.

pub mod components;
pub mod profile;
pub mod job_search;
pub mod resume_builder;
pub mod views;
pub mod shared_navigation;

// Re-export main components for easy access
pub use components::Dashboard;
pub use components::ResumeUpload;
pub use components::Hero;
pub use components::Navbar;
pub use components::Echo;

// Re-export additional components
pub use profile::ProfileManager;
pub use job_search::JobSearch;
pub use resume_builder::ResumeBuilder;

// Re-export views
pub use views::{Dashboard as DashboardView, Profile as ProfileView, Resumes as ResumesView, Jobs as JobsView, Applications as ApplicationsView};
pub use shared_navigation::SharedNavigation;


// Custom Label component since dioxus-bootstrap doesn't have one
use dioxus::prelude::*;

#[component]
pub fn Label(
    #[props(optional)] r#for: Option<String>,
    #[props(optional, default = "".to_string())] class: String,
    children: Element
) -> Element {
    rsx! {
        label {
            r#for: r#for,
            class: class,
            {children}
        }
    }
}

// Re-export dioxus-bootstrap components for convenience  
pub use dioxus_bootstrap::{
    Container, Row, Col, 
    Button, ButtonVariant, ButtonGroup, Size,
    Card, CardBody, CardHeader, CardFooter,
    Form, Input, InputType, Textarea, Select, Checkbox, Radio,
    Modal, ModalHeader, ModalBody, ModalFooter, ModalSize,
    Alert, AlertVariant,
    Badge, BadgeVariant,
    Dropdown, DropdownMenu, DropdownItem
};
