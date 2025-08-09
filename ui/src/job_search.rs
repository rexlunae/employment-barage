use dioxus::prelude::*;
use dioxus_bootstrap::*;
use api::{Job, JobSource, search_jobs, generate_cover_letter, apply_to_job, JobApplication, ApplicationStatus};
use api::job_service::CoverLetterTone;

#[component]
pub fn JobSearch() -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut location = use_signal(|| "".to_string());
    let mut min_salary = use_signal(|| None::<u32>);
    let mut selected_sources = use_signal(|| vec![JobSource::LinkedIn, JobSource::Indeed]);
    let mut jobs = use_signal(|| Vec::<Job>::new());
    let mut is_searching = use_signal(|| false);
    let mut selected_job = use_signal(|| None::<Job>);
    let mut show_application_modal = use_signal(|| false);

    let handle_search = move |_| {
        spawn(async move {
            is_searching.set(true);
            match search_jobs(
                search_query(),
                location(),
                min_salary(),
                selected_sources()
            ).await {
                Ok(results) => {
                    jobs.set(results);
                }
                Err(e) => {
                    tracing::error!("Search failed: {}", e);
                }
            }
            is_searching.set(false);
        });
    };

    rsx! {
        Container { fluid: true,
            Row {
                Col { lg: 4,
                    Card {
                        CardHeader {
                            h4 { "Job Search" }
                        }
                        CardBody {
                            Form {
                                div { class: "mb-3",
                                    Label { r#for: "search-query", class: "form-label", "Keywords" }
                                    Input {
                                        r#type: "text",
                                        id: "search-query",
                                        class: "form-control",
                                        placeholder: "e.g. Software Engineer, Data Scientist",
                                        value: search_query(),
                                        oninput: move |evt| search_query.set(evt.value())
                                    }
                                }
                                
                                div { class: "mb-3",
                                    Label { r#for: "location", class: "form-label", "Location" }
                                    Input {
                                        r#type: "text",
                                        id: "location",
                                        class: "form-control",
                                        placeholder: "City, State or Remote",
                                        value: location(),
                                        oninput: move |evt| location.set(evt.value())
                                    }
                                }
                                
                                div { class: "mb-3",
                                    Label { r#for: "min-salary", class: "form-label", "Minimum Salary (USD)" }
                                    Input {
                                        r#type: "number",
                                        id: "min-salary",
                                        class: "form-control",
                                        placeholder: "e.g. 80000",
                                        value: min_salary().map(|s| s.to_string()).unwrap_or_default(),
                                        oninput: move |evt| {
                                            let val = evt.value();
                                            min_salary.set(val.parse::<u32>().ok());
                                        }
                                    }
                                }
                                
                                div { class: "mb-3",
                                    Label { class: "form-label", "Job Sources" }
                                    div { class: "form-check",
                                        input {
                                            class: "form-check-input",
                                            r#type: "checkbox",
                                            id: "linkedin",
                                            checked: selected_sources().contains(&JobSource::LinkedIn),
                                            onchange: move |evt| {
                                                let mut sources = selected_sources();
                                                if evt.checked() {
                                                    if !sources.contains(&JobSource::LinkedIn) {
                                                        sources.push(JobSource::LinkedIn);
                                                    }
                                                } else {
                                                    sources.retain(|s| s != &JobSource::LinkedIn);
                                                }
                                                selected_sources.set(sources);
                                            }
                                        }
                                        Label { r#for: "linkedin", class: "form-check-label", "LinkedIn" }
                                    }
                                    div { class: "form-check",
                                        input {
                                            class: "form-check-input",
                                            r#type: "checkbox",
                                            id: "indeed",
                                            checked: selected_sources().contains(&JobSource::Indeed),
                                            onchange: move |evt| {
                                                let mut sources = selected_sources();
                                                if evt.checked() {
                                                    if !sources.contains(&JobSource::Indeed) {
                                                        sources.push(JobSource::Indeed);
                                                    }
                                                } else {
                                                    sources.retain(|s| s != &JobSource::Indeed);
                                                }
                                                selected_sources.set(sources);
                                            }
                                        }
                                        Label { r#for: "indeed", class: "form-check-label", "Indeed" }
                                    }
                                    div { class: "form-check",
                                        input {
                                            class: "form-check-input",
                                            r#type: "checkbox",
                                            id: "glassdoor",
                                            checked: selected_sources().contains(&JobSource::Glassdoor),
                                            onchange: move |evt| {
                                                let mut sources = selected_sources();
                                                if evt.checked() {
                                                    if !sources.contains(&JobSource::Glassdoor) {
                                                        sources.push(JobSource::Glassdoor);
                                                    }
                                                } else {
                                                    sources.retain(|s| s != &JobSource::Glassdoor);
                                                }
                                                selected_sources.set(sources);
                                            }
                                        }
                                        Label { r#for: "glassdoor", class: "form-check-label", "Glassdoor" }
                                    }
                                }
                                
                                div { class: "d-grid",
                                    Button {
                                        color: ButtonColor::Primary,
                                        size: ButtonSize::Large,
                                        disabled: is_searching(),
                                        onclick: handle_search,
                                        if is_searching() {
                                            span { class: "spinner-border spinner-border-sm me-2" }
                                            "Searching..."
                                        } else {
                                            span {
                                                i { class: "fas fa-search me-2" }
                                                "Search Jobs"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Col { lg: 8,
                    div { class: "d-flex justify-content-between align-items-center mb-3",
                        h4 { "Search Results" }
                        if !jobs().is_empty() {
                            Badge { color: BadgeColor::Info, "{jobs().len()} jobs found" }
                        }
                    }
                    
                    if jobs().is_empty() && !is_searching() {
                        Card {
                            CardBody {
                                div { class: "text-center text-muted py-5",
                                    i { class: "fas fa-search fa-4x mb-3" }
                                    h5 { "Start your job search" }
                                    p { "Enter keywords and location to find relevant opportunities" }
                                }
                            }
                        }
                    } else {
                        div { class: "job-results",
                            for job in jobs() {
                                JobCard { 
                                    job: job.clone(),
                                    on_apply: move |j| {
                                        selected_job.set(Some(j));
                                        show_application_modal.set(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            if show_application_modal() && selected_job().is_some() {
                ApplicationModal { 
                    job: selected_job().unwrap(),
                    on_close: move |_| {
                        show_application_modal.set(false);
                        selected_job.set(None);
                    }
                }
            }
        }
    }
}

#[component]
fn JobCard(job: Job, on_apply: EventHandler<Job>) -> Element {
    let source_color = match job.source {
        JobSource::LinkedIn => "primary",
        JobSource::Indeed => "success", 
        JobSource::Glassdoor => "info",
        JobSource::AngelList => "warning",
        JobSource::Other(_) => "secondary",
    };

    let source_name = match &job.source {
        JobSource::LinkedIn => "LinkedIn",
        JobSource::Indeed => "Indeed",
        JobSource::Glassdoor => "Glassdoor", 
        JobSource::AngelList => "AngelList",
        JobSource::Other(name) => name,
    };

    rsx! {
        Card { class: "mb-3 hover-shadow",
            CardBody {
                div { class: "d-flex justify-content-between align-items-start mb-3",
                    div {
                        h5 { class: "mb-1", 
                            a { 
                                href: job.source_url.clone(),
                                target: "_blank",
                                class: "text-decoration-none",
                                "{job.title}"
                            }
                        }
                        h6 { class: "text-primary mb-2", "{job.company}" }
                        p { class: "text-muted mb-2",
                            i { class: "fas fa-map-marker-alt me-1" }
                            "{job.location}"
                        }
                    }
                    div { class: "text-end",
                        Badge { 
                            color: BadgeColor::from_str(source_color).unwrap_or(BadgeColor::Secondary),
                            class: "mb-2",
                            "{source_name}"
                        }
                        if let Some(salary) = &job.salary_range {
                            div { class: "text-success fw-bold",
                                "${salary.min:,} - ${salary.max:,}"
                            }
                        }
                    }
                }
                
                p { class: "mb-3", "{job.description}" }
                
                if !job.requirements.is_empty() {
                    div { class: "mb-3",
                        h6 { "Requirements:" }
                        ul { class: "list-unstyled mb-0",
                            for req in job.requirements.iter().take(3) {
                                li { class: "mb-1",
                                    i { class: "fas fa-check text-success me-2" }
                                    "{req}"
                                }
                            }
                            if job.requirements.len() > 3 {
                                li { class: "text-muted",
                                    "... and {job.requirements.len() - 3} more requirements"
                                }
                            }
                        }
                    }
                }
                
                div { class: "d-flex justify-content-between align-items-center",
                    small { class: "text-muted",
                        "Posted {job.posted_date.format(\"%B %d, %Y\")}"
                    }
                    div {
                        Button {
                            color: ButtonColor::Outline(OutlineColor::Primary),
                            size: ButtonSize::Small,
                            class: "me-2",
                            i { class: "fas fa-external-link-alt me-1" }
                            "View"
                        }
                        Button {
                            color: ButtonColor::Primary,
                            size: ButtonSize::Small,
                            onclick: move |_| on_apply.call(job.clone()),
                            i { class: "fas fa-paper-plane me-1" }
                            "Quick Apply"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ApplicationModal(job: Job, on_close: EventHandler<()>) -> Element {
    let mut cover_letter = use_signal(|| "".to_string());
    let mut tone = use_signal(|| CoverLetterTone::Professional);
    let mut is_generating = use_signal(|| false);
    let mut is_applying = use_signal(|| false);
    let mut auto_submit = use_signal(|| false);

    let generate_cover_letter_handler = move |_| {
        spawn(async move {
            is_generating.set(true);
            match generate_cover_letter(
                job.id.to_string(),
                "dummy_profile_id".to_string(),
                tone()
            ).await {
                Ok(letter) => {
                    cover_letter.set(letter);
                }
                Err(e) => {
                    tracing::error!("Failed to generate cover letter: {}", e);
                }
            }
            is_generating.set(false);
        });
    };

    let submit_application = move |_| {
        spawn(async move {
            is_applying.set(true);
            match apply_to_job(
                job.id.to_string(),
                "dummy_user_id".to_string(),
                cover_letter(),
                "dummy_resume_id".to_string(),
                auto_submit()
            ).await {
                Ok(_application) => {
                    // Show success message
                    on_close.call(());
                }
                Err(e) => {
                    tracing::error!("Failed to submit application: {}", e);
                }
            }
            is_applying.set(false);
        });
    };

    rsx! {
        div { 
            class: "modal d-block",
            style: "background: rgba(0,0,0,0.5);",
            div { class: "modal-dialog modal-lg modal-dialog-centered",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h5 { class: "modal-title", "Apply to {job.title}" }
                        button {
                            r#type: "button",
                            class: "btn-close",
                            onclick: move |_| on_close.call(())
                        }
                    }
                    div { class: "modal-body",
                        div { class: "mb-3",
                            Label { class: "form-label", "Cover Letter Tone" }
                            select {
                                class: "form-select",
                                value: format!("{:?}", tone()),
                                onchange: move |evt| {
                                    tone.set(match evt.value().as_str() {
                                        "Professional" => CoverLetterTone::Professional,
                                        "Friendly" => CoverLetterTone::Friendly,
                                        "Enthusiastic" => CoverLetterTone::Enthusiastic,
                                        _ => CoverLetterTone::Professional,
                                    });
                                },
                                option { value: "Professional", "Professional" }
                                option { value: "Friendly", "Friendly" }
                                option { value: "Enthusiastic", "Enthusiastic" }
                            }
                        }
                        
                        div { class: "mb-3",
                            div { class: "d-flex justify-content-between align-items-center mb-2",
                                Label { class: "form-label", "Cover Letter" }
                                Button {
                                    color: ButtonColor::Outline(OutlineColor::Primary),
                                    size: ButtonSize::Small,
                                    disabled: is_generating(),
                                    onclick: generate_cover_letter_handler,
                                    if is_generating() {
                                        span { class: "spinner-border spinner-border-sm me-1" }
                                        "Generating..."
                                    } else {
                                        span {
                                            i { class: "fas fa-magic me-1" }
                                            "Generate"
                                        }
                                    }
                                }
                            }
                            textarea {
                                class: "form-control",
                                rows: "8",
                                placeholder: "Your personalized cover letter will appear here...",
                                value: cover_letter(),
                                oninput: move |evt| cover_letter.set(evt.value())
                            }
                        }
                        
                        div { class: "form-check mb-3",
                            input {
                                class: "form-check-input",
                                r#type: "checkbox",
                                id: "auto-submit",
                                checked: auto_submit(),
                                onchange: move |evt| auto_submit.set(evt.checked())
                            }
                            Label { r#for: "auto-submit", class: "form-check-label",
                                "Automatically submit application (when possible)"
                            }
                        }
                    }
                    div { class: "modal-footer",
                        Button {
                            color: ButtonColor::Secondary,
                            onclick: move |_| on_close.call(()),
                            "Cancel"
                        }
                        Button {
                            color: ButtonColor::Primary,
                            disabled: is_applying() || cover_letter().trim().is_empty(),
                            onclick: submit_application,
                            if is_applying() {
                                span { class: "spinner-border spinner-border-sm me-2" }
                                "Applying..."
                            } else {
                                span {
                                    i { class: "fas fa-paper-plane me-2" }
                                    if auto_submit() { "Apply Now" } else { "Save Application" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}