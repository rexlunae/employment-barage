use dioxus::prelude::*;
use dioxus_bootstrap::*;
use api::{ResumeTemplate, generate_resume, analyze_resume, ResumeAnalysis};
use crate::Label;

#[component]
pub fn ResumeBuilder() -> Element {
    let mut selected_template = use_signal(|| ResumeTemplate::Professional);
    let mut preview_html = use_signal(|| String::new());
    let mut is_generating = use_signal(|| false);
    let mut analysis = use_signal(|| None::<ResumeAnalysis>);

    let handle_generate = move |_| {
        spawn(async move {
            is_generating.set(true);
            match generate_resume(
                "dummy_profile_id".to_string(),
                selected_template(),
                vec![], // selected experiences
                vec![], // selected projects
                vec![], // selected skills
            ).await {
                Ok(html) => {
                    preview_html.set(html);
                }
                Err(e) => {
                    tracing::error!("Failed to generate resume: {}", e);
                }
            }
            is_generating.set(false);
        });
    };

    let handle_analyze = move |_| {
        spawn(async move {
            match analyze_resume("dummy_resume_id".to_string()).await {
                Ok(result) => {
                    analysis.set(Some(result));
                }
                Err(e) => {
                    tracing::error!("Failed to analyze resume: {}", e);
                }
            }
        });
    };

    rsx! {
        Container {
            Row {
                Col { lg: 4,
                    Card {
                        CardHeader {
                            h4 { "Resume Builder" }
                        }
                        CardBody {
                            div { class: "mb-4",
                                Label { class: "form-label fw-bold", "Template Style" }
                                div { class: "template-grid",
                                    for template in [
                                        ResumeTemplate::Professional,
                                        ResumeTemplate::Modern,
                                        ResumeTemplate::Creative,
                                        ResumeTemplate::Simple,
                                        ResumeTemplate::Academic,
                                    ] {
                                        TemplateCard { 
                                            template: template.clone(),
                                            selected: selected_template() == template,
                                            on_select: move |t| selected_template.set(t)
                                        }
                                    }
                                }
                            }
                            
                            div { class: "mb-4",
                                Label { class: "form-label fw-bold", "Content Selection" }
                                Checkbox {
                                    id: "include-summary",
                                    label: Some("Professional Summary".to_string()),
                                    checked: true
                                }
                                Checkbox {
                                    id: "include-experience", 
                                    label: Some("Work Experience".to_string()),
                                    checked: true
                                }
                                Checkbox {
                                    id: "include-education",
                                    label: Some("Education".to_string()),
                                    checked: true
                                }
                                Checkbox {
                                    id: "include-skills",
                                    label: Some("Skills".to_string()),
                                    checked: true
                                }
                                Checkbox {
                                    id: "include-projects",
                                    label: Some("Projects".to_string()),
                                    checked: true
                                }
                            }
                            
                            div { class: "d-grid gap-2",
                                Button {
                                    variant: ButtonVariant::Primary,
                                    size: Size::Large,
                                    disabled: is_generating(),
                                    onclick: handle_generate,
                                    if is_generating() {
                                        span { class: "spinner-border spinner-border-sm me-2" }
                                        "Generating..."
                                    } else {
                                        span {
                                            i { class: "fas fa-magic me-2" }
                                            "Generate Resume"
                                        }
                                    }
                                }
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    onclick: handle_analyze,
                                    i { class: "fas fa-chart-line me-2" }
                                    "Analyze Resume"
                                }
                            }
                        }
                    }
                    
                    if let Some(analysis_data) = analysis() {
                        AnalysisCard { analysis: analysis_data }
                    }
                }
                Col { lg: 8,
                    Card {
                        CardHeader {
                            div { class: "d-flex justify-content-between align-items-center",
                                h4 { "Resume Preview" }
                                div {
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        size: Size::Small,
                                        i { class: "fas fa-download me-1" }
                                        "Download PDF"
                                    }
                                }
                            }
                        }
                        CardBody {
                            if preview_html().is_empty() {
                                div { class: "text-center text-muted py-5",
                                    i { class: "fas fa-file-alt fa-4x mb-3" }
                                    h5 { "No resume generated yet" }
                                    p { "Select a template and click 'Generate Resume' to see the preview" }
                                }
                            } else {
                                div { 
                                    class: "resume-preview border rounded p-4 bg-white",
                                    style: "min-height: 800px; box-shadow: 0 0 20px rgba(0,0,0,0.1);",
                                    dangerous_inner_html: preview_html()
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
fn TemplateCard(
    template: ResumeTemplate,
    selected: bool,
    on_select: EventHandler<ResumeTemplate>
) -> Element {
    let (name, description, preview_class) = match template {
        ResumeTemplate::Professional => ("Professional", "Clean and traditional", "professional-preview"),
        ResumeTemplate::Modern => ("Modern", "Contemporary design", "modern-preview"),
        ResumeTemplate::Creative => ("Creative", "Bold and artistic", "creative-preview"),
        ResumeTemplate::Simple => ("Simple", "Minimalist approach", "simple-preview"),
        ResumeTemplate::Academic => ("Academic", "Scholarly format", "academic-preview"),
    };

    rsx! {
        div { 
            class: format!("template-card border rounded p-3 mb-2 cursor-pointer {}", 
                if selected { "border-primary bg-primary bg-opacity-10" } else { "border-light" }),
            onclick: move |_| on_select.call(template.clone()),
            div { class: format!("template-preview {} mb-2", preview_class),
                style: "height: 80px; background: #f8f9fa; border-radius: 4px; position: relative;",
                div { class: "template-lines",
                    div { class: "line", style: "width: 60%; height: 3px; background: #dee2e6; margin: 8px;" }
                    div { class: "line", style: "width: 80%; height: 2px; background: #dee2e6; margin: 8px;" }
                    div { class: "line", style: "width: 40%; height: 2px; background: #dee2e6; margin: 8px;" }
                    div { class: "line", style: "width: 70%; height: 2px; background: #dee2e6; margin: 8px;" }
                }
            }
            h6 { class: "mb-1", "{name}" }
            p { class: "text-muted small mb-0", "{description}" }
        }
    }
}

#[component]
fn AnalysisCard(analysis: ResumeAnalysis) -> Element {
    let score_color = if analysis.score >= 80 {
        "success"
    } else if analysis.score >= 60 {
        "warning"  
    } else {
        "danger"
    };

    rsx! {
        Card { class: "mt-3",
            CardHeader {
                h5 { "Resume Analysis" }
            }
            CardBody {
                div { class: "mb-3",
                    div { class: "d-flex justify-content-between align-items-center mb-2",
                        span { "Overall Score" }
                        span { class: "badge bg-primary", 
                            {format!("{}/100", analysis.score)}
                        }
                    }
                    div { class: "progress",
                        div { 
                            class: format!("progress-bar bg-{}", score_color),
                            style: format!("width: {}%", analysis.score),
                            role: "progressbar"
                        }
                    }
                }
                
                div { class: "mb-3",
                    small { class: "text-muted", 
                        {format!("ATS Compatibility: {:.1}%", analysis.ats_compatibility * 100.0)}
                    }
                }
                
                if !analysis.suggestions.is_empty() {
                    div {
                        h6 { "Suggestions for Improvement:" }
                        for suggestion in &analysis.suggestions {
                            Alert { 
                                variant: match suggestion.priority {
                                    api::Priority::Critical => AlertVariant::Danger,
                                    api::Priority::High => AlertVariant::Warning, 
                                    api::Priority::Medium => AlertVariant::Info,
                                    api::Priority::Low => AlertVariant::Light,
                                },
                                class: "py-2",
                                small { class: "fw-bold text-uppercase", 
                                    {format!("{:?}", suggestion.category)}
                                }
                                div { {suggestion.message.clone()} }
                            }
                        }
                    }
                }
            }
        }
    }
}