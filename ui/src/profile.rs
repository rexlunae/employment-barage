use dioxus::prelude::*;
use dioxus_bootstrap::*;
use api::{Profile, Experience, Education, Skill, Project, SkillCategory, SkillLevel};
use uuid::Uuid;
use chrono::Utc;

#[component]
pub fn ProfileManager(profile: Signal<Option<Profile>>) -> Element {
    let mut active_tab = use_signal(|| ProfileTab::Personal);
    
    rsx! {
        Container { fluid: true,
            Row {
                Col { lg: 3,
                    Card {
                        CardBody {
                            h5 { class: "mb-3", "Profile Sections" }
                            Nav { pills: true, vertical: true,
                                NavItem {
                                    NavLink {
                                        href: "#",
                                        active: active_tab() == ProfileTab::Personal,
                                        onclick: move |_| active_tab.set(ProfileTab::Personal),
                                        i { class: "fas fa-user me-2" }
                                        "Personal Info"
                                    }
                                }
                                NavItem {
                                    NavLink {
                                        href: "#",
                                        active: active_tab() == ProfileTab::Experience,
                                        onclick: move |_| active_tab.set(ProfileTab::Experience),
                                        i { class: "fas fa-briefcase me-2" }
                                        "Experience"
                                    }
                                }
                                NavItem {
                                    NavLink {
                                        href: "#",
                                        active: active_tab() == ProfileTab::Education,
                                        onclick: move |_| active_tab.set(ProfileTab::Education),
                                        i { class: "fas fa-graduation-cap me-2" }
                                        "Education"
                                    }
                                }
                                NavItem {
                                    NavLink {
                                        href: "#",
                                        active: active_tab() == ProfileTab::Skills,
                                        onclick: move |_| active_tab.set(ProfileTab::Skills),
                                        i { class: "fas fa-cog me-2" }
                                        "Skills"
                                    }
                                }
                                NavItem {
                                    NavLink {
                                        href: "#",
                                        active: active_tab() == ProfileTab::Projects,
                                        onclick: move |_| active_tab.set(ProfileTab::Projects),
                                        i { class: "fas fa-code me-2" }
                                        "Projects"
                                    }
                                }
                            }
                        }
                    }
                }
                Col { lg: 9,
                    match active_tab() {
                        ProfileTab::Personal => rsx! { PersonalInfoForm { profile } },
                        ProfileTab::Experience => rsx! { ExperienceManager {} },
                        ProfileTab::Education => rsx! { EducationManager {} },
                        ProfileTab::Skills => rsx! { SkillsManager {} },
                        ProfileTab::Projects => rsx! { ProjectsManager {} },
                    }
                }
            }
        }
    }
}

#[component]
fn PersonalInfoForm(profile: Signal<Option<Profile>>) -> Element {
    let mut form_data = use_signal(|| {
        profile().unwrap_or_else(|| Profile {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            summary: None,
            phone: None,
            email: "".to_string(),
            location: None,
            linkedin_url: None,
            github_url: None,
            portfolio_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    });

    let handle_save = move |_| {
        spawn(async move {
            match api::save_profile(form_data()).await {
                Ok(saved_profile) => {
                    profile.set(Some(saved_profile));
                }
                Err(e) => {
                    // Handle error
                    tracing::error!("Failed to save profile: {}", e);
                }
            }
        });
    };

    rsx! {
        Card {
            CardHeader {
                h4 { "Personal Information" }
            }
            CardBody {
                Form {
                    Row {
                        Col { md: 6,
                            div { class: "mb-3",
                                Label { r#for: "email", class: "form-label", "Email Address" }
                                Input {
                                    r#type: "email",
                                    id: "email",
                                    class: "form-control",
                                    value: form_data().email,
                                    oninput: move |evt| {
                                        let mut data = form_data();
                                        data.email = evt.value();
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                        Col { md: 6,
                            div { class: "mb-3",
                                Label { r#for: "phone", class: "form-label", "Phone Number" }
                                Input {
                                    r#type: "tel",
                                    id: "phone",
                                    class: "form-control",
                                    value: form_data().phone.unwrap_or_default(),
                                    oninput: move |evt| {
                                        let mut data = form_data();
                                        data.phone = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                    }
                    
                    div { class: "mb-3",
                        Label { r#for: "location", class: "form-label", "Location" }
                        Input {
                            r#type: "text",
                            id: "location",
                            class: "form-control",
                            placeholder: "City, State/Country",
                            value: form_data().location.unwrap_or_default(),
                            oninput: move |evt| {
                                let mut data = form_data();
                                data.location = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                form_data.set(data);
                            }
                        }
                    }
                    
                    div { class: "mb-3",
                        Label { r#for: "summary", class: "form-label", "Professional Summary" }
                        textarea {
                            id: "summary",
                            class: "form-control",
                            rows: "4",
                            placeholder: "Write a brief summary of your professional background and goals...",
                            value: form_data().summary.unwrap_or_default(),
                            oninput: move |evt| {
                                let mut data = form_data();
                                data.summary = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                form_data.set(data);
                            }
                        }
                    }
                    
                    Row {
                        Col { md: 4,
                            div { class: "mb-3",
                                Label { r#for: "linkedin", class: "form-label", "LinkedIn URL" }
                                Input {
                                    r#type: "url",
                                    id: "linkedin",
                                    class: "form-control",
                                    placeholder: "https://linkedin.com/in/yourprofile",
                                    value: form_data().linkedin_url.unwrap_or_default(),
                                    oninput: move |evt| {
                                        let mut data = form_data();
                                        data.linkedin_url = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                        Col { md: 4,
                            div { class: "mb-3",
                                Label { r#for: "github", class: "form-label", "GitHub URL" }
                                Input {
                                    r#type: "url",
                                    id: "github",
                                    class: "form-control",
                                    placeholder: "https://github.com/yourusername",
                                    value: form_data().github_url.unwrap_or_default(),
                                    oninput: move |evt| {
                                        let mut data = form_data();
                                        data.github_url = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                        Col { md: 4,
                            div { class: "mb-3",
                                Label { r#for: "portfolio", class: "form-label", "Portfolio URL" }
                                Input {
                                    r#type: "url",
                                    id: "portfolio",
                                    class: "form-control",
                                    placeholder: "https://yourportfolio.com",
                                    value: form_data().portfolio_url.unwrap_or_default(),
                                    oninput: move |evt| {
                                        let mut data = form_data();
                                        data.portfolio_url = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                    }
                    
                    div { class: "d-flex justify-content-end",
                        Button {
                            color: ButtonColor::Primary,
                            size: ButtonSize::Large,
                            onclick: handle_save,
                            i { class: "fas fa-save me-2" }
                            "Save Changes"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExperienceManager() -> Element {
    let mut experiences = use_signal(|| Vec::<Experience>::new());
    let mut show_form = use_signal(|| false);

    rsx! {
        Card {
            CardHeader {
                div { class: "d-flex justify-content-between align-items-center",
                    h4 { "Work Experience" }
                    Button {
                        color: ButtonColor::Primary,
                        onclick: move |_| show_form.set(true),
                        i { class: "fas fa-plus me-2" }
                        "Add Experience"
                    }
                }
            }
            CardBody {
                if show_form() {
                    ExperienceForm { 
                        on_save: move |exp| {
                            let mut exps = experiences();
                            exps.push(exp);
                            experiences.set(exps);
                            show_form.set(false);
                        },
                        on_cancel: move |_| show_form.set(false)
                    }
                }
                
                if experiences().is_empty() && !show_form() {
                    div { class: "text-center text-muted py-5",
                        i { class: "fas fa-briefcase fa-3x mb-3" }
                        h5 { "No experience added yet" }
                        p { "Click 'Add Experience' to get started" }
                    }
                } else {
                    for experience in experiences() {
                        ExperienceCard { experience: experience.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ExperienceForm(on_save: EventHandler<Experience>, on_cancel: EventHandler<()>) -> Element {
    let mut form_data = use_signal(|| Experience {
        id: Uuid::new_v4(),
        profile_id: Uuid::new_v4(),
        company: String::new(),
        position: String::new(),
        location: None,
        start_date: Utc::now(),
        end_date: None,
        current: false,
        description: String::new(),
        achievements: Vec::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });

    rsx! {
        Card { class: "mb-3 border-primary",
            CardBody {
                h5 { class: "mb-3", "Add Work Experience" }
                Form {
                    Row {
                        Col { md: 6,
                            div { class: "mb-3",
                                Label { r#for: "company", class: "form-label", "Company *" }
                                Input {
                                    r#type: "text",
                                    id: "company",
                                    class: "form-control",
                                    required: true,
                                    value: form_data().company,
                                    oninput: move |evt| {
                                        let mut data = form_data();
                                        data.company = evt.value();
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                        Col { md: 6,
                            div { class: "mb-3",
                                Label { r#for: "position", class: "form-label", "Position *" }
                                Input {
                                    r#type: "text",
                                    id: "position",
                                    class: "form-control",
                                    required: true,
                                    value: form_data().position,
                                    oninput: move |evt| {
                                        let mut data = form_data();
                                        data.position = evt.value();
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                    }
                    
                    div { class: "mb-3",
                        Label { r#for: "description", class: "form-label", "Description" }
                        textarea {
                            id: "description",
                            class: "form-control",
                            rows: "4",
                            placeholder: "Describe your role, responsibilities, and achievements...",
                            value: form_data().description,
                            oninput: move |evt| {
                                let mut data = form_data();
                                data.description = evt.value();
                                form_data.set(data);
                            }
                        }
                    }
                    
                    div { class: "d-flex justify-content-end gap-2",
                        Button {
                            color: ButtonColor::Secondary,
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                        Button {
                            color: ButtonColor::Primary,
                            onclick: move |_| {
                                on_save.call(form_data());
                            },
                            "Save Experience"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExperienceCard(experience: Experience) -> Element {
    rsx! {
        Card { class: "mb-3",
            CardBody {
                div { class: "d-flex justify-content-between align-items-start",
                    div {
                        h5 { class: "mb-1", "{experience.position}" }
                        h6 { class: "text-primary mb-2", "{experience.company}" }
                        if let Some(location) = &experience.location {
                            p { class: "text-muted mb-2", 
                                i { class: "fas fa-map-marker-alt me-1" }
                                "{location}"
                            }
                        }
                        p { class: "mb-2", "{experience.description}" }
                    }
                    div { class: "text-end",
                        Button {
                            color: ButtonColor::Outline(OutlineColor::Secondary),
                            size: ButtonSize::Small,
                            i { class: "fas fa-edit" }
                        }
                    }
                }
            }
        }
    }
}

// Simplified versions of other managers for brevity
#[component]
fn EducationManager() -> Element {
    rsx! {
        Card {
            CardHeader { h4 { "Education" } }
            CardBody {
                div { class: "text-center text-muted py-5",
                    i { class: "fas fa-graduation-cap fa-3x mb-3" }
                    h5 { "Education manager coming soon" }
                }
            }
        }
    }
}

#[component]
fn SkillsManager() -> Element {
    rsx! {
        Card {
            CardHeader { h4 { "Skills & Competencies" } }
            CardBody {
                div { class: "text-center text-muted py-5",
                    i { class: "fas fa-cog fa-3x mb-3" }
                    h5 { "Skills manager coming soon" }
                }
            }
        }
    }
}

#[component]
fn ProjectsManager() -> Element {
    rsx! {
        Card {
            CardHeader { h4 { "Projects" } }
            CardBody {
                div { class: "text-center text-muted py-5",
                    i { class: "fas fa-code fa-3x mb-3" }
                    h5 { "Projects manager coming soon" }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum ProfileTab {
    Personal,
    Experience,
    Education,
    Skills,
    Projects,
}