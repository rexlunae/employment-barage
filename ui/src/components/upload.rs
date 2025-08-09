use dioxus::prelude::*;
use dioxus_bootstrap::*;
use api::{upload_resume, ParsedResume};

#[component]
pub fn ResumeUpload(on_upload: EventHandler<ParsedResume>) -> Element {
    let mut upload_status = use_signal(|| UploadStatus::Ready);

    let handle_file_upload = move |evt: FormEvent| {
        spawn(async move {
            if let Some(file_engine) = evt.files() {
                let files = file_engine.files();
                if let Some(file) = files.get(0) {
                    upload_status.set(UploadStatus::Uploading);
                    
                    let file_name = file.clone();
                    // Convert file to bytes - simplified for now
                    let file_data = vec![]; // TODO: implement file reading
                    
                    match upload_resume(file_data, file_name, "dummy_user_id".to_string()).await {
                        Ok(parsed_resume) => {
                            upload_status.set(UploadStatus::Success);
                            on_upload.call(parsed_resume);
                        }
                        Err(e) => {
                            upload_status.set(UploadStatus::Error(e.to_string()));
                        }
                    }
                }
            }
        });
    };

    rsx! {
        Container {
            Row {
                Col { md: 8, offset_md: 2,
                    Card {
                        CardBody {
                            h3 { class: "mb-4", "Upload Your Resume" }
                            p { class: "text-muted mb-4", 
                                "Upload your resume in PDF or DOCX format. Our system will automatically extract and organize your information."
                            }
                            
                            match upload_status() {
                                UploadStatus::Ready => rsx! {
                                    div { class: "upload-zone border border-dashed border-primary rounded p-5 text-center mb-3",
                                        input {
                                            r#type: "file",
                                            accept: ".pdf,.docx",
                                            class: "form-control",
                                            id: "resume-upload",
                                            onchange: handle_file_upload
                                        }
                                        label {
                                            r#for: "resume-upload",
                                            class: "d-block mt-3",
                                            div { class: "mb-3",
                                                i { class: "fas fa-cloud-upload-alt fa-3x text-primary" }
                                            }
                                            h5 { "Click to upload or drag and drop" }
                                            p { class: "text-muted", "Supported formats: PDF, DOCX" }
                                        }
                                    }
                                },
                                UploadStatus::Uploading => rsx! {
                                    div { class: "text-center p-5",
                                        div { class: "spinner-border text-primary mb-3", role: "status" }
                                        h5 { "Processing your resume..." }
                                        p { class: "text-muted", "This may take a few moments while we extract your information." }
                                    }
                                },
                                UploadStatus::Success => rsx! {
                                    div { class: "alert alert-success",
                                        div { class: "d-flex align-items-center",
                                            i { class: "fas fa-check-circle me-2" }
                                            div {
                                                strong { "Success! " }
                                                "Your resume has been processed and your profile has been updated."
                                            }
                                        }
                                    }
                                },
                                UploadStatus::Error(err) => rsx! {
                                    div { class: "alert alert-danger",
                                        div { class: "d-flex align-items-center",
                                            i { class: "fas fa-exclamation-triangle me-2" }
                                            div {
                                                strong { "Error: " }
                                                "{err}"
                                            }
                                        }
                                    }
                                    Button { 
                                        variant: ButtonVariant::Primary, 
                                        class: "mt-2",
                                        onclick: move |_| upload_status.set(UploadStatus::Ready),
                                        "Try Again"
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

#[derive(Clone, PartialEq)]
enum UploadStatus {
    Ready,
    Uploading,
    Success,
    Error(String),
}