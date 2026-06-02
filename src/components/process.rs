use crate::util::use_scroll_animation;
use dioxus::prelude::*;

struct ProcessStep {
    number: &'static str,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Process() -> Element {
    use_scroll_animation(".process-section", ".process-step", 0.7, 40.0, 0.1, 0.0);

    let steps: Vec<ProcessStep> = vec![
        ProcessStep {
            number: "01",
            title: "UNDERSTAND",
            description: "Before touching a file, I read. The brief, the brand, the users. Assumptions are bugs introduced before the editor opens.",
        },
        ProcessStep {
            number: "02",
            title: "ARCHITECT",
            description: "Rust forces you to think about ownership before you build. I apply the same principle to systems design. No shortcuts that cost later.",
        },
        ProcessStep {
            number: "03",
            title: "BUILD",
            description: "Dioxus compiles to WASM. The build step is the contract. Everything after it is real.",
        },
        ProcessStep {
            number: "04",
            title: "SHIP",
            description: "Deployed, measured, and handed off with documentation. Done means done — not 'working on my machine'.",
        },
    ];

    rsx! {
        section {
            id: "process",
            class: "process-section",
            div {
                class: "max-w-content mx-auto",
                div {
                    class: "section-header",
                    div {
                        class: "section-number",
                        "04"
                    }
                    span {
                        class: "section-label",
                        "PROCESS"
                    }
                }
                div {
                    class: "process-steps",
                    for (i, step) in steps.iter().enumerate() {
                        div {
                            class: "process-step",
                            div {
                                class: "process-step-number",
                                "{step.number}"
                            }
                            h3 {
                                class: "process-step-title",
                                "{step.title}"
                            }
                            p {
                                class: "process-step-desc",
                                "{step.description}"
                            }
                        }
                        if i < steps.len() - 1 {
                            div {
                                class: "process-divider",
                            }
                        }
                    }
                }
            }
        }
    }
}
