use dioxus::prelude::*;
use dioxus_bootstrap::*;
use api::{upload_resume, ParsedResume};
use web_sys::{File, FileReader};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn ResumeUpload(on_upload: EventHandler<ParsedResume>) -> Element {
    let mut upload_status = use_signal(|| UploadStatus::Ready);
    let mut progress = use_signal(|| 0.0);

    let handle_file_upload = move |evt: FormEvent| {
        spawn(async move {
            if let Some(file_engine) = evt.files() {
                if let Some(files) = file_engine.files() {
                    if let Some(file) = files.get(0) {
                        upload_status.set(UploadStatus::Uploading);
                        
                        match read_file_as_bytes(file).await {
                            Ok(file_data) => {
                                let file_name = file.name();
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
                            Err(e) => {
                                upload_status.set(UploadStatus::Error(e));
                            }
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
                                    div { class: "upload-zone border-2 border-dashed border-primary rounded p-5 text-center mb-3",
                                        style: "cursor: pointer; transition: all 0.3s ease;",
                                        input {
                                            r#type: "file",
                                            accept: ".pdf,.docx",
                                            class: "form-control",
                                            style: "display: none;",
                                            id: "resume-upload",
                                            onchange: handle_file_upload
                                        }
                                        label {
                                            r#for: "resume-upload",
                                            class: "d-block",
                                            style: "cursor: pointer;",
                                            div { class: "mb-3",
                                                i { class: "fas fa-cloud-upload-alt fa-3x text-primary" }
                                            }
                                            h5 { "Click to upload or drag and drop" }
                                            p { class: "text-muted", "Supported formats: PDF, DOCX" }
                                        }
                                    }
                                }
                                UploadStatus::Uploading => rsx! {
                                    div { class: "text-center p-5",
                                        div { class: "spinner-border text-primary mb-3", role: "status" }
                                        h5 { "Processing your resume..." }
                                        p { class: "text-muted", "This may take a few moments while we extract your information." }
                                    }
                                }
                                UploadStatus::Success => rsx! {
                                    Alert { color: AlertColor::Success,
                                        div { class: "d-flex align-items-center",
                                            i { class: "fas fa-check-circle me-2" }
                                            div {
                                                strong { "Success! " }
                                                "Your resume has been processed and your profile has been updated."
                                            }
                                        }
                                    }
                                }
                                UploadStatus::Error(err) => rsx! {
                                    Alert { color: AlertColor::Danger,
                                        div { class: "d-flex align-items-center",
                                            i { class: "fas fa-exclamation-triangle me-2" }
                                            div {
                                                strong { "Error: " }
                                                "{err}"
                                            }
                                        }
                                    }
                                    Button {
                                        color: ButtonColor::Primary,
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

async fn read_file_as_bytes(file: File) -> Result<Vec<u8>, String> {
    let file_reader = FileReader::new().map_err(|_| "Failed to create FileReader")?;
    let file_reader_clone = file_reader.clone();
    
    let (sender, receiver) = futures::channel::oneshot::channel();
    let mut sender = Some(sender);
    
    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
        if let Some(sender) = sender.take() {
            let result = file_reader_clone.result().unwrap();
            let array = js_sys::Uint8Array::new(&result);
            let bytes = array.to_vec();
            sender.send(Ok(bytes)).unwrap();
        }
    }) as Box<dyn FnMut(_)>);
    
    let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
        // Handle error case
    }) as Box<dyn FnMut(_)>);
    
    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    
    file_reader.read_as_array_buffer(&file).map_err(|_| "Failed to read file")?;
    
    let result = receiver.await.map_err(|_| "Failed to receive file data")?;
    
    onload.forget();
    onerror.forget();
    
    result
}