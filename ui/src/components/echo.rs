use dioxus::prelude::*;
use dioxus_bootstrap::*;

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        Container {
            id: "echo",
            class: "py-4",
            Row {
                Col { md: 8, offset_md: 2,
                    Card {
                        CardHeader {
                            h4 { class: "mb-0", "ServerFn Echo" }
                        }
                        CardBody {
                            Form {
                                div { class: "mb-3",
                                    Input {
                                        input_type: InputType::Text,
                                        class: "form-control",
                                        placeholder: "Type here to echo...",
                                        oninput: move |event: Event<FormData>| async move {
                                            let data = api::echo(event.value()).await.unwrap();
                                            response.set(data);
                                        },
                                    }
                                }

                                if !response().is_empty() {
                                    Alert { variant: AlertVariant::Info,
                                        strong { "Server echoed: " }
                                        span { class: "fst-italic", "{response}" }
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