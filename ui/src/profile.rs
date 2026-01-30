//! Profile Management UI Component
//! 
//! A comprehensive profile editor with sections for:
//! - Personal information (name, email, phone, location)
//! - Professional summary
//! - Skills (with categories and proficiency levels)
//! - Work history
//! - Education

use dioxus::prelude::*;
use dioxus_bootstrap::*;
use api::{Profile, Experience, Education, Skill, SkillCategory, SkillLevel};
use uuid::Uuid;
use chrono::Utc;
use crate::Label;

/// Main profile manager component with tabbed navigation
#[component]
pub fn ProfileManager(profile: Signal<Option<Profile>>) -> Element {
    let active_tab = use_signal(|| ProfileTab::Personal);
    let profile_id = use_memo(move || profile().map(|p| p.id));
    
    // Load profile data on mount
    use_effect(move || {
        spawn(async move {
            // Try to load existing profile for a default user
            // In a real app, this would come from auth
            let user_id = "00000000-0000-0000-0000-000000000001".to_string();
            if let Ok(Some(loaded_profile)) = api::get_profile(user_id).await {
                profile.set(Some(loaded_profile));
            }
        });
    });
    
    rsx! {
        Container {
            Row {
                Col { lg: 3,
                    div { 
                        class: "sticky-top",
                        style: "top: 20px;",
                        Card {
                            CardBody {
                                h5 { class: "mb-3", 
                                    i { class: "fas fa-user-circle me-2 text-primary" }
                                    "Profile Sections" 
                                }
                                div { class: "nav nav-pills flex-column gap-1",
                                    ProfileNavButton {
                                        icon: "fas fa-user",
                                        label: "Personal Info",
                                        tab: ProfileTab::Personal,
                                        active_tab: active_tab
                                    }
                                    ProfileNavButton {
                                        icon: "fas fa-briefcase",
                                        label: "Experience",
                                        tab: ProfileTab::Experience,
                                        active_tab: active_tab
                                    }
                                    ProfileNavButton {
                                        icon: "fas fa-graduation-cap",
                                        label: "Education",
                                        tab: ProfileTab::Education,
                                        active_tab: active_tab
                                    }
                                    ProfileNavButton {
                                        icon: "fas fa-cogs",
                                        label: "Skills",
                                        tab: ProfileTab::Skills,
                                        active_tab: active_tab
                                    }
                                }
                                
                                // Profile completeness indicator
                                if profile().is_some() {
                                    hr { class: "my-3" }
                                    ProfileCompleteness { profile: profile }
                                }
                            }
                        }
                    }
                }
                Col { lg: 9,
                    match active_tab() {
                        ProfileTab::Personal => rsx! { PersonalInfoForm { profile } },
                        ProfileTab::Experience => rsx! { ExperienceManager { profile_id } },
                        ProfileTab::Education => rsx! { EducationManager { profile_id } },
                        ProfileTab::Skills => rsx! { SkillsManager { profile_id } },
                    }
                }
            }
        }
    }
}

#[component]
fn ProfileNavButton(
    icon: &'static str,
    label: &'static str,
    tab: ProfileTab,
    active_tab: Signal<ProfileTab>
) -> Element {
    let is_active = active_tab() == tab;
    
    rsx! {
        button {
            class: format!("nav-link text-start {}", if is_active { "active" } else { "" }),
            onclick: move |_| active_tab.set(tab.clone()),
            i { class: format!("{} me-2", icon) }
            {label}
        }
    }
}

#[component]
fn ProfileCompleteness(profile: Signal<Option<Profile>>) -> Element {
    let completeness = use_memo(move || {
        if let Some(p) = profile() {
            let mut score = 0;
            let total = 5;
            
            if !p.name.is_empty() { score += 1; }
            if !p.email.is_empty() { score += 1; }
            if p.phone.is_some() { score += 1; }
            if p.location.is_some() { score += 1; }
            if p.summary.is_some() { score += 1; }
            
            (score * 100) / total
        } else {
            0
        }
    });
    
    let badge_color = if completeness() >= 80 {
        "bg-success"
    } else if completeness() >= 50 {
        "bg-warning"
    } else {
        "bg-danger"
    };
    
    rsx! {
        div { class: "small",
            div { class: "d-flex justify-content-between mb-1",
                span { class: "text-muted", "Profile Complete" }
                span { class: format!("badge {}", badge_color), "{completeness()}%" }
            }
            div { class: "progress",
                style: "height: 6px;",
                div { 
                    class: format!("progress-bar {}", badge_color),
                    style: format!("width: {}%", completeness()),
                    role: "progressbar"
                }
            }
        }
    }
}

/// Personal information form
#[component]
fn PersonalInfoForm(profile: Signal<Option<Profile>>) -> Element {
    let mut form_data = use_signal(|| {
        profile().unwrap_or_else(|| Profile {
            id: Uuid::new_v4(),
            user_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            name: String::new(),
            email: String::new(),
            headline: None,
            summary: None,
            phone: None,
            location: None,
            linkedin_url: None,
            github_url: None,
            portfolio_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    });
    
    let mut is_saving = use_signal(|| false);
    let mut save_status = use_signal(|| None::<SaveStatus>);

    // Update form when profile changes
    use_effect(move || {
        if let Some(p) = profile() {
            form_data.set(p);
        }
    });

    let handle_save = move |_| {
        spawn(async move {
            is_saving.set(true);
            save_status.set(None);
            
            let mut updated = form_data();
            updated.updated_at = Utc::now();
            
            match api::save_profile(updated.clone()).await {
                Ok(saved_profile) => {
                    profile.set(Some(saved_profile.clone()));
                    form_data.set(saved_profile);
                    save_status.set(Some(SaveStatus::Success));
                }
                Err(e) => {
                    tracing::error!("Failed to save profile: {}", e);
                    save_status.set(Some(SaveStatus::Error(e.to_string())));
                }
            }
            is_saving.set(false);
        });
    };

    rsx! {
        Card {
            CardHeader { class: "bg-white",
                div { class: "d-flex justify-content-between align-items-center",
                    h4 { class: "mb-0",
                        i { class: "fas fa-user me-2 text-primary" }
                        "Personal Information"
                    }
                    if let Some(status) = save_status() {
                        match status {
                            SaveStatus::Success => rsx! {
                                Badge { variant: BadgeVariant::Success,
                                    i { class: "fas fa-check me-1" }
                                    "Saved"
                                }
                            },
                            SaveStatus::Error(msg) => rsx! {
                                Badge { variant: BadgeVariant::Danger,
                                    i { class: "fas fa-exclamation-triangle me-1" }
                                    "Error: {msg}"
                                }
                            },
                        }
                    }
                }
            }
            CardBody {
                Form {
                    // Name and Email Row
                    Row { class: "mb-3",
                        Col { md: 6,
                            Label { r#for: "name", class: "form-label fw-semibold", 
                                "Full Name "
                                span { class: "text-danger", "*" }
                            }
                            Input {
                                input_type: InputType::Text,
                                id: "name",
                                class: "form-control",
                                placeholder: "John Doe",
                                value: form_data().name,
                                oninput: move |evt: Event<FormData>| {
                                    let mut data = form_data();
                                    data.name = evt.value();
                                    form_data.set(data);
                                }
                            }
                        }
                        Col { md: 6,
                            Label { r#for: "email", class: "form-label fw-semibold", 
                                "Email Address "
                                span { class: "text-danger", "*" }
                            }
                            Input {
                                input_type: InputType::Email,
                                id: "email",
                                class: "form-control",
                                placeholder: "john@example.com",
                                value: form_data().email,
                                oninput: move |evt: Event<FormData>| {
                                    let mut data = form_data();
                                    data.email = evt.value();
                                    form_data.set(data);
                                }
                            }
                        }
                    }
                    
                    // Phone and Location Row
                    Row { class: "mb-3",
                        Col { md: 6,
                            Label { r#for: "phone", class: "form-label fw-semibold", "Phone Number" }
                            div { class: "input-group",
                                span { class: "input-group-text",
                                    i { class: "fas fa-phone" }
                                }
                                Input {
                                    input_type: InputType::Tel,
                                    id: "phone",
                                    class: "form-control",
                                    placeholder: "+1 (555) 123-4567",
                                    value: form_data().phone.clone().unwrap_or_default(),
                                    oninput: move |evt: Event<FormData>| {
                                        let mut data = form_data();
                                        data.phone = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                        Col { md: 6,
                            Label { r#for: "location", class: "form-label fw-semibold", "Location" }
                            div { class: "input-group",
                                span { class: "input-group-text",
                                    i { class: "fas fa-map-marker-alt" }
                                }
                                Input {
                                    input_type: InputType::Text,
                                    id: "location",
                                    class: "form-control",
                                    placeholder: "San Francisco, CA",
                                    value: form_data().location.clone().unwrap_or_default(),
                                    oninput: move |evt: Event<FormData>| {
                                        let mut data = form_data();
                                        data.location = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                    }
                    
                    // Headline
                    div { class: "mb-3",
                        Label { r#for: "headline", class: "form-label fw-semibold", "Professional Headline" }
                        Input {
                            input_type: InputType::Text,
                            id: "headline",
                            class: "form-control",
                            placeholder: "Senior Software Engineer | Full-Stack Developer | Tech Lead",
                            value: form_data().headline.clone().unwrap_or_default(),
                            oninput: move |evt: Event<FormData>| {
                                let mut data = form_data();
                                data.headline = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                form_data.set(data);
                            }
                        }
                        small { class: "text-muted", "A brief tagline that summarizes your professional identity" }
                    }
                    
                    // Summary
                    div { class: "mb-3",
                        Label { r#for: "summary", class: "form-label fw-semibold", "Professional Summary" }
                        Textarea {
                            id: "summary",
                            class: "form-control",
                            rows: 5,
                            placeholder: "Write a compelling summary of your professional background, key achievements, and career goals...",
                            value: form_data().summary.clone().unwrap_or_default(),
                            oninput: move |evt: Event<FormData>| {
                                let mut data = form_data();
                                data.summary = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                form_data.set(data);
                            }
                        }
                        small { class: "text-muted", "This summary will be used by AI to generate personalized cover letters" }
                    }
                    
                    // Social Links
                    h5 { class: "mt-4 mb-3",
                        i { class: "fas fa-link me-2 text-muted" }
                        "Online Presence"
                    }
                    Row { class: "mb-3",
                        Col { md: 4,
                            Label { r#for: "linkedin", class: "form-label fw-semibold", "LinkedIn" }
                            div { class: "input-group",
                                span { class: "input-group-text",
                                    i { class: "fab fa-linkedin" }
                                }
                                Input {
                                    input_type: InputType::Url,
                                    id: "linkedin",
                                    class: "form-control",
                                    placeholder: "linkedin.com/in/yourprofile",
                                    value: form_data().linkedin_url.clone().unwrap_or_default(),
                                    oninput: move |evt: Event<FormData>| {
                                        let mut data = form_data();
                                        data.linkedin_url = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                        Col { md: 4,
                            Label { r#for: "github", class: "form-label fw-semibold", "GitHub" }
                            div { class: "input-group",
                                span { class: "input-group-text",
                                    i { class: "fab fa-github" }
                                }
                                Input {
                                    input_type: InputType::Url,
                                    id: "github",
                                    class: "form-control",
                                    placeholder: "github.com/username",
                                    value: form_data().github_url.clone().unwrap_or_default(),
                                    oninput: move |evt: Event<FormData>| {
                                        let mut data = form_data();
                                        data.github_url = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                        Col { md: 4,
                            Label { r#for: "portfolio", class: "form-label fw-semibold", "Portfolio" }
                            div { class: "input-group",
                                span { class: "input-group-text",
                                    i { class: "fas fa-globe" }
                                }
                                Input {
                                    input_type: InputType::Url,
                                    id: "portfolio",
                                    class: "form-control",
                                    placeholder: "yourportfolio.com",
                                    value: form_data().portfolio_url.clone().unwrap_or_default(),
                                    oninput: move |evt: Event<FormData>| {
                                        let mut data = form_data();
                                        data.portfolio_url = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        form_data.set(data);
                                    }
                                }
                            }
                        }
                    }
                    
                    // Save Button
                    div { class: "d-flex justify-content-end mt-4",
                        Button {
                            variant: ButtonVariant::Primary,
                            size: Size::Large,
                            disabled: is_saving(),
                            onclick: handle_save,
                            if is_saving() {
                                span { class: "spinner-border spinner-border-sm me-2" }
                                "Saving..."
                            } else {
                                span {
                                    i { class: "fas fa-save me-2" }
                                    "Save Changes"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Experience management section
#[component]
fn ExperienceManager(profile_id: Memo<Option<Uuid>>) -> Element {
    let mut experiences = use_signal(|| Vec::<Experience>::new());
    let mut show_form = use_signal(|| false);
    let mut editing_id = use_signal(|| None::<Uuid>);
    let mut is_loading = use_signal(|| true);

    // Load experiences
    use_effect(move || {
        if let Some(pid) = profile_id() {
            spawn(async move {
                is_loading.set(true);
                if let Ok(loaded) = api::get_experiences(pid.to_string()).await {
                    experiences.set(loaded);
                }
                is_loading.set(false);
            });
        }
    });

    let handle_save = move |exp: Experience| {
        spawn(async move {
            match api::save_experience(exp.clone()).await {
                Ok(saved) => {
                    let mut exps = experiences();
                    if let Some(idx) = exps.iter().position(|e| e.id == saved.id) {
                        exps[idx] = saved;
                    } else {
                        exps.insert(0, saved);
                    }
                    experiences.set(exps);
                    show_form.set(false);
                    editing_id.set(None);
                }
                Err(e) => {
                    tracing::error!("Failed to save experience: {}", e);
                }
            }
        });
    };

    let handle_delete = move |exp_id: Uuid| {
        spawn(async move {
            if let Ok(()) = api::delete_experience(exp_id.to_string()).await {
                let mut exps = experiences();
                exps.retain(|e| e.id != exp_id);
                experiences.set(exps);
            }
        });
    };

    rsx! {
        Card {
            CardHeader { class: "bg-white",
                div { class: "d-flex justify-content-between align-items-center",
                    h4 { class: "mb-0",
                        i { class: "fas fa-briefcase me-2 text-primary" }
                        "Work Experience"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            editing_id.set(None);
                            show_form.set(true);
                        },
                        i { class: "fas fa-plus me-2" }
                        "Add Experience"
                    }
                }
            }
            CardBody {
                if is_loading() {
                    div { class: "text-center py-5",
                        div { class: "spinner-border text-primary" }
                        p { class: "mt-2 text-muted", "Loading experiences..." }
                    }
                } else if show_form() {
                    ExperienceForm { 
                        profile_id: profile_id().unwrap_or(Uuid::nil()),
                        experience: editing_id().and_then(|id| experiences().iter().find(|e| e.id == id).cloned()),
                        on_save: handle_save,
                        on_cancel: move |_| {
                            show_form.set(false);
                            editing_id.set(None);
                        }
                    }
                } else if experiences().is_empty() {
                    div { class: "text-center text-muted py-5",
                        i { class: "fas fa-briefcase fa-4x mb-3 opacity-25" }
                        h5 { "No experience added yet" }
                        p { "Click 'Add Experience' to showcase your work history" }
                    }
                } else {
                    for experience in experiences() {
                        ExperienceCard { 
                            experience: experience.clone(),
                            on_edit: move |id| {
                                editing_id.set(Some(id));
                                show_form.set(true);
                            },
                            on_delete: handle_delete
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExperienceForm(
    profile_id: Uuid,
    experience: Option<Experience>,
    on_save: EventHandler<Experience>,
    on_cancel: EventHandler<()>
) -> Element {
    let is_edit = experience.is_some();
    let mut form_data = use_signal(move || {
        experience.clone().unwrap_or_else(|| Experience {
            id: Uuid::new_v4(),
            profile_id,
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
        })
    });

    rsx! {
        Card { class: "mb-3 border-primary",
            CardBody {
                h5 { class: "mb-3", 
                    if is_edit { "Edit Experience" } else { "Add Work Experience" }
                }
                Form {
                    Row { class: "mb-3",
                        Col { md: 6,
                            Label { r#for: "company", class: "form-label fw-semibold", 
                                "Company "
                                span { class: "text-danger", "*" }
                            }
                            Input {
                                input_type: InputType::Text,
                                id: "company",
                                class: "form-control",
                                placeholder: "Company Name",
                                value: form_data().company,
                                oninput: move |evt: Event<FormData>| {
                                    let mut data = form_data();
                                    data.company = evt.value();
                                    form_data.set(data);
                                }
                            }
                        }
                        Col { md: 6,
                            Label { r#for: "position", class: "form-label fw-semibold", 
                                "Position "
                                span { class: "text-danger", "*" }
                            }
                            Input {
                                input_type: InputType::Text,
                                id: "position",
                                class: "form-control",
                                placeholder: "Job Title",
                                value: form_data().position,
                                oninput: move |evt: Event<FormData>| {
                                    let mut data = form_data();
                                    data.position = evt.value();
                                    form_data.set(data);
                                }
                            }
                        }
                    }
                    
                    div { class: "mb-3",
                        Label { r#for: "exp-location", class: "form-label fw-semibold", "Location" }
                        Input {
                            input_type: InputType::Text,
                            id: "exp-location",
                            class: "form-control",
                            placeholder: "City, State or Remote",
                            value: form_data().location.clone().unwrap_or_default(),
                            oninput: move |evt: Event<FormData>| {
                                let mut data = form_data();
                                data.location = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                form_data.set(data);
                            }
                        }
                    }
                    
                    div { class: "mb-3",
                        Checkbox {
                            id: "current-job",
                            label: Some("I currently work here".to_string()),
                            checked: form_data().current,
                            onchange: move |evt: Event<FormData>| {
                                let mut data = form_data();
                                data.current = evt.checked();
                                if data.current {
                                    data.end_date = None;
                                }
                                form_data.set(data);
                            }
                        }
                    }
                    
                    div { class: "mb-3",
                        Label { r#for: "description", class: "form-label fw-semibold", "Description" }
                        Textarea {
                            id: "description",
                            class: "form-control",
                            rows: 4,
                            placeholder: "Describe your role, responsibilities, and key achievements...",
                            value: form_data().description,
                            oninput: move |evt: Event<FormData>| {
                                let mut data = form_data();
                                data.description = evt.value();
                                form_data.set(data);
                            }
                        }
                    }
                    
                    div { class: "d-flex justify-content-end gap-2",
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            disabled: form_data().company.is_empty() || form_data().position.is_empty(),
                            onclick: move |_| {
                                let mut data = form_data();
                                data.updated_at = Utc::now();
                                on_save.call(data);
                            },
                            i { class: "fas fa-save me-2" }
                            "Save Experience"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExperienceCard(
    experience: Experience,
    on_edit: EventHandler<Uuid>,
    on_delete: EventHandler<Uuid>
) -> Element {
    let exp_id = experience.id;
    
    rsx! {
        Card { class: "mb-3 border-start border-primary border-3",
            CardBody {
                div { class: "d-flex justify-content-between align-items-start",
                    div { class: "flex-grow-1",
                        div { class: "d-flex align-items-center gap-2 mb-1",
                            h5 { class: "mb-0", "{experience.position}" }
                            if experience.current {
                                Badge { variant: BadgeVariant::Success, "Current" }
                            }
                        }
                        h6 { class: "text-primary mb-2", "{experience.company}" }
                        if let Some(location) = &experience.location {
                            p { class: "text-muted mb-2 small", 
                                i { class: "fas fa-map-marker-alt me-1" }
                                "{location}"
                            }
                        }
                        if !experience.description.is_empty() {
                            p { class: "mb-2", "{experience.description}" }
                        }
                    }
                    div { class: "btn-group",
                        Button {
                            variant: ButtonVariant::Primary,
                            outline: true,
                            size: Size::Small,
                            onclick: move |_| on_edit.call(exp_id),
                            i { class: "fas fa-edit" }
                        }
                        Button {
                            variant: ButtonVariant::Danger,
                            outline: true,
                            size: Size::Small,
                            onclick: move |_| on_delete.call(exp_id),
                            i { class: "fas fa-trash" }
                        }
                    }
                }
            }
        }
    }
}

/// Education management section
#[component]
fn EducationManager(profile_id: Memo<Option<Uuid>>) -> Element {
    let mut education_list = use_signal(|| Vec::<Education>::new());
    let mut show_form = use_signal(|| false);
    let mut editing_id = use_signal(|| None::<Uuid>);
    let mut is_loading = use_signal(|| true);

    // Load education
    use_effect(move || {
        if let Some(pid) = profile_id() {
            spawn(async move {
                is_loading.set(true);
                if let Ok(loaded) = api::get_education(pid.to_string()).await {
                    education_list.set(loaded);
                }
                is_loading.set(false);
            });
        }
    });

    let handle_save = move |edu: Education| {
        spawn(async move {
            match api::save_education(edu.clone()).await {
                Ok(saved) => {
                    let mut list = education_list();
                    if let Some(idx) = list.iter().position(|e| e.id == saved.id) {
                        list[idx] = saved;
                    } else {
                        list.insert(0, saved);
                    }
                    education_list.set(list);
                    show_form.set(false);
                    editing_id.set(None);
                }
                Err(e) => {
                    tracing::error!("Failed to save education: {}", e);
                }
            }
        });
    };

    let handle_delete = move |edu_id: Uuid| {
        spawn(async move {
            if let Ok(()) = api::delete_education(edu_id.to_string()).await {
                let mut list = education_list();
                list.retain(|e| e.id != edu_id);
                education_list.set(list);
            }
        });
    };

    rsx! {
        Card {
            CardHeader { class: "bg-white",
                div { class: "d-flex justify-content-between align-items-center",
                    h4 { class: "mb-0",
                        i { class: "fas fa-graduation-cap me-2 text-primary" }
                        "Education"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            editing_id.set(None);
                            show_form.set(true);
                        },
                        i { class: "fas fa-plus me-2" }
                        "Add Education"
                    }
                }
            }
            CardBody {
                if is_loading() {
                    div { class: "text-center py-5",
                        div { class: "spinner-border text-primary" }
                        p { class: "mt-2 text-muted", "Loading education..." }
                    }
                } else if show_form() {
                    EducationForm { 
                        profile_id: profile_id().unwrap_or(Uuid::nil()),
                        education: editing_id().and_then(|id| education_list().iter().find(|e| e.id == id).cloned()),
                        on_save: handle_save,
                        on_cancel: move |_| {
                            show_form.set(false);
                            editing_id.set(None);
                        }
                    }
                } else if education_list().is_empty() {
                    div { class: "text-center text-muted py-5",
                        i { class: "fas fa-graduation-cap fa-4x mb-3 opacity-25" }
                        h5 { "No education added yet" }
                        p { "Click 'Add Education' to add your academic background" }
                    }
                } else {
                    for edu in education_list() {
                        EducationCard { 
                            education: edu.clone(),
                            on_edit: move |id| {
                                editing_id.set(Some(id));
                                show_form.set(true);
                            },
                            on_delete: handle_delete
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EducationForm(
    profile_id: Uuid,
    education: Option<Education>,
    on_save: EventHandler<Education>,
    on_cancel: EventHandler<()>
) -> Element {
    let is_edit = education.is_some();
    let mut form_data = use_signal(move || {
        education.clone().unwrap_or_else(|| Education {
            id: Uuid::new_v4(),
            profile_id,
            institution: String::new(),
            degree: String::new(),
            field: String::new(),
            location: None,
            start_date: Utc::now(),
            end_date: None,
            gpa: None,
            honors: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    });

    rsx! {
        Card { class: "mb-3 border-primary",
            CardBody {
                h5 { class: "mb-3", 
                    if is_edit { "Edit Education" } else { "Add Education" }
                }
                Form {
                    div { class: "mb-3",
                        Label { r#for: "institution", class: "form-label fw-semibold", 
                            "Institution "
                            span { class: "text-danger", "*" }
                        }
                        Input {
                            input_type: InputType::Text,
                            id: "institution",
                            class: "form-control",
                            placeholder: "University Name",
                            value: form_data().institution,
                            oninput: move |evt: Event<FormData>| {
                                let mut data = form_data();
                                data.institution = evt.value();
                                form_data.set(data);
                            }
                        }
                    }
                    
                    Row { class: "mb-3",
                        Col { md: 6,
                            Label { r#for: "degree", class: "form-label fw-semibold", 
                                "Degree "
                                span { class: "text-danger", "*" }
                            }
                            Input {
                                input_type: InputType::Text,
                                id: "degree",
                                class: "form-control",
                                placeholder: "Bachelor of Science",
                                value: form_data().degree,
                                oninput: move |evt: Event<FormData>| {
                                    let mut data = form_data();
                                    data.degree = evt.value();
                                    form_data.set(data);
                                }
                            }
                        }
                        Col { md: 6,
                            Label { r#for: "field", class: "form-label fw-semibold", 
                                "Field of Study "
                                span { class: "text-danger", "*" }
                            }
                            Input {
                                input_type: InputType::Text,
                                id: "field",
                                class: "form-control",
                                placeholder: "Computer Science",
                                value: form_data().field,
                                oninput: move |evt: Event<FormData>| {
                                    let mut data = form_data();
                                    data.field = evt.value();
                                    form_data.set(data);
                                }
                            }
                        }
                    }
                    
                    Row { class: "mb-3",
                        Col { md: 6,
                            Label { r#for: "gpa", class: "form-label fw-semibold", "GPA" }
                            Input {
                                input_type: InputType::Text,
                                id: "gpa",
                                class: "form-control",
                                placeholder: "3.8",
                                value: form_data().gpa.map(|g| g.to_string()).unwrap_or_default(),
                                oninput: move |evt: Event<FormData>| {
                                    let mut data = form_data();
                                    data.gpa = evt.value().parse().ok();
                                    form_data.set(data);
                                }
                            }
                        }
                    }
                    
                    div { class: "d-flex justify-content-end gap-2",
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            disabled: form_data().institution.is_empty() || form_data().degree.is_empty(),
                            onclick: move |_| {
                                let mut data = form_data();
                                data.updated_at = Utc::now();
                                on_save.call(data);
                            },
                            i { class: "fas fa-save me-2" }
                            "Save Education"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EducationCard(
    education: Education,
    on_edit: EventHandler<Uuid>,
    on_delete: EventHandler<Uuid>
) -> Element {
    let edu_id = education.id;
    
    rsx! {
        Card { class: "mb-3 border-start border-success border-3",
            CardBody {
                div { class: "d-flex justify-content-between align-items-start",
                    div { class: "flex-grow-1",
                        h5 { class: "mb-1", "{education.degree}" }
                        h6 { class: "text-primary mb-2", "{education.institution}" }
                        p { class: "text-muted mb-1", "{education.field}" }
                        if let Some(gpa) = education.gpa {
                            Badge { variant: BadgeVariant::Info, "GPA: {gpa}" }
                        }
                    }
                    div { class: "btn-group",
                        Button {
                            variant: ButtonVariant::Primary,
                            outline: true,
                            size: Size::Small,
                            onclick: move |_| on_edit.call(edu_id),
                            i { class: "fas fa-edit" }
                        }
                        Button {
                            variant: ButtonVariant::Danger,
                            outline: true,
                            size: Size::Small,
                            onclick: move |_| on_delete.call(edu_id),
                            i { class: "fas fa-trash" }
                        }
                    }
                }
            }
        }
    }
}

/// Skills management section
#[component]
fn SkillsManager(profile_id: Memo<Option<Uuid>>) -> Element {
    let mut skills = use_signal(|| Vec::<Skill>::new());
    let mut new_skill_name = use_signal(|| String::new());
    let mut new_skill_category = use_signal(|| SkillCategory::Programming);
    let mut new_skill_level = use_signal(|| SkillLevel::Intermediate);
    let mut is_loading = use_signal(|| true);

    // Load skills
    use_effect(move || {
        if let Some(pid) = profile_id() {
            spawn(async move {
                is_loading.set(true);
                if let Ok(loaded) = api::get_skills(pid.to_string()).await {
                    skills.set(loaded);
                }
                is_loading.set(false);
            });
        }
    });

    let handle_add_skill = move |_| {
        if let Some(pid) = profile_id() {
            let name = new_skill_name();
            if name.is_empty() {
                return;
            }
            
            let skill = Skill {
                id: Uuid::new_v4(),
                profile_id: pid,
                name,
                category: new_skill_category(),
                proficiency: new_skill_level(),
                years_experience: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            spawn(async move {
                match api::save_skill(skill.clone()).await {
                    Ok(saved) => {
                        let mut current = skills();
                        current.push(saved);
                        skills.set(current);
                        new_skill_name.set(String::new());
                    }
                    Err(e) => {
                        tracing::error!("Failed to save skill: {}", e);
                    }
                }
            });
        }
    };

    let handle_delete = move |skill_id: Uuid| {
        spawn(async move {
            if let Ok(()) = api::delete_skill(skill_id.to_string()).await {
                let mut current = skills();
                current.retain(|s| s.id != skill_id);
                skills.set(current);
            }
        });
    };

    // Group skills by category
    let skills_by_category = use_memo(move || {
        let mut grouped: std::collections::HashMap<String, Vec<Skill>> = std::collections::HashMap::new();
        for skill in skills() {
            let category = format!("{:?}", skill.category);
            grouped.entry(category).or_default().push(skill);
        }
        grouped
    });

    rsx! {
        Card {
            CardHeader { class: "bg-white",
                h4 { class: "mb-0",
                    i { class: "fas fa-cogs me-2 text-primary" }
                    "Skills & Competencies"
                }
            }
            CardBody {
                // Add skill form
                Card { class: "mb-4 bg-light",
                    CardBody {
                        h6 { class: "mb-3", "Add New Skill" }
                        Row { class: "g-2 align-items-end",
                            Col { md: 4,
                                Label { r#for: "skill-name", class: "form-label small", "Skill Name" }
                                Input {
                                    input_type: InputType::Text,
                                    id: "skill-name",
                                    class: "form-control",
                                    placeholder: "e.g., Python, React, Project Management",
                                    value: new_skill_name(),
                                    oninput: move |evt: Event<FormData>| new_skill_name.set(evt.value())
                                }
                            }
                            Col { md: 3,
                                Label { r#for: "skill-category", class: "form-label small", "Category" }
                                Select {
                                    id: "skill-category",
                                    class: "form-select",
                                    value: format!("{:?}", new_skill_category()),
                                    onchange: move |evt: Event<FormData>| {
                                        new_skill_category.set(match evt.value().as_str() {
                                            "Programming" => SkillCategory::Programming,
                                            "Framework" => SkillCategory::Framework,
                                            "Database" => SkillCategory::Database,
                                            "Tool" => SkillCategory::Tool,
                                            "Language" => SkillCategory::Language,
                                            "Soft" => SkillCategory::Soft,
                                            _ => SkillCategory::Other,
                                        });
                                    },
                                    option { value: "Programming", "Programming" }
                                    option { value: "Framework", "Framework" }
                                    option { value: "Database", "Database" }
                                    option { value: "Tool", "Tool" }
                                    option { value: "Language", "Language" }
                                    option { value: "Soft", "Soft Skills" }
                                    option { value: "Other", "Other" }
                                }
                            }
                            Col { md: 3,
                                Label { r#for: "skill-level", class: "form-label small", "Proficiency" }
                                Select {
                                    id: "skill-level",
                                    class: "form-select",
                                    value: format!("{:?}", new_skill_level()),
                                    onchange: move |evt: Event<FormData>| {
                                        new_skill_level.set(match evt.value().as_str() {
                                            "Beginner" => SkillLevel::Beginner,
                                            "Intermediate" => SkillLevel::Intermediate,
                                            "Advanced" => SkillLevel::Advanced,
                                            "Expert" => SkillLevel::Expert,
                                            _ => SkillLevel::Intermediate,
                                        });
                                    },
                                    option { value: "Beginner", "Beginner" }
                                    option { value: "Intermediate", "Intermediate" }
                                    option { value: "Advanced", "Advanced" }
                                    option { value: "Expert", "Expert" }
                                }
                            }
                            Col { md: 2,
                                Button {
                                    variant: ButtonVariant::Primary,
                                    class: "w-100",
                                    disabled: new_skill_name().is_empty(),
                                    onclick: handle_add_skill,
                                    i { class: "fas fa-plus me-1" }
                                    "Add"
                                }
                            }
                        }
                    }
                }
                
                // Skills display
                if is_loading() {
                    div { class: "text-center py-5",
                        div { class: "spinner-border text-primary" }
                        p { class: "mt-2 text-muted", "Loading skills..." }
                    }
                } else if skills().is_empty() {
                    div { class: "text-center text-muted py-5",
                        i { class: "fas fa-cogs fa-4x mb-3 opacity-25" }
                        h5 { "No skills added yet" }
                        p { "Add your skills to highlight your expertise" }
                    }
                } else {
                    for (category, category_skills) in skills_by_category() {
                        div { class: "mb-4",
                            h6 { class: "text-muted mb-3",
                                i { class: "fas fa-folder me-2" }
                                "{category}"
                            }
                            div { class: "d-flex flex-wrap gap-2",
                                for skill in category_skills {
                                    SkillTag { 
                                        skill: skill.clone(),
                                        on_delete: handle_delete
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillTag(skill: Skill, on_delete: EventHandler<Uuid>) -> Element {
    let skill_id = skill.id;
    let level_color = match skill.proficiency {
        SkillLevel::Beginner => "secondary",
        SkillLevel::Intermediate => "info",
        SkillLevel::Advanced => "primary",
        SkillLevel::Expert => "success",
    };
    
    rsx! {
        span { 
            class: format!("badge bg-{} d-flex align-items-center gap-2 py-2 px-3", level_color),
            style: "font-size: 0.9rem;",
            "{skill.name}"
            button {
                class: "btn-close btn-close-white",
                style: "font-size: 0.6rem;",
                onclick: move |_| on_delete.call(skill_id),
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
}

#[derive(Clone)]
enum SaveStatus {
    Success,
    Error(String),
}
