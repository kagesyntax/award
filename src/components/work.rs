use crate::util::use_scroll_animation;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Project {
    number: &'static str,
    name: &'static str,
    project_type: &'static str,
    year: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    coming_soon: bool,
}

#[component]
pub fn Work() -> Element {
    use_scroll_animation(".work-section", ".work-card", 0.8, 50.0, 0.12, 0.0);

    let projects: Vec<Project> = vec![
        Project {
            number: "01",
            name: "PORTFOLIO OS",
            project_type: "Personal Project",
            year: "2025",
            description: "The site you're looking at. Built with Dioxus and compiled to WebAssembly. No JavaScript framework. No React. Just Rust, running in the browser at native speed.",
            tags: &["Rust", "Dioxus", "WASM", "TailwindCSS"],
            coming_soon: false,
        },
        Project {
            number: "02",
            name: "AXUM FULLSTACK STARTER",
            project_type: "Open Source Template",
            year: "2025",
            description: "A production-ready Dioxus fullstack template. Axum backend, server-side rendering, SQLite with Turso, and deployment config for Railway. Built because the tooling documentation assumed you already knew.",
            tags: &["Rust", "Dioxus", "Axum", "Turso", "Railway"],
            coming_soon: false,
        },
        Project {
            number: "03",
            name: "CLIENT SITE",
            project_type: "Coming Soon",
            year: "2025",
            description: "A web presence for a brand that needed something that looked nothing like anything else. Currently in design with my collaborator. Launching when it's ready — not before.",
            tags: &["Dioxus", "GSAP", "TailwindCSS", "Cloudflare Pages"],
            coming_soon: true,
        },
    ];

    rsx! {
        section {
            id: "work",
            class: "work-section",
            div {
                class: "max-w-content mx-auto",
                div {
                    class: "section-header",
                    div {
                        class: "section-number",
                        "03"
                    }
                    span {
                        class: "section-label",
                        "WORK"
                    }
                }
                div {
                    class: "work-list",
                    for project in projects {
                        ProjectCard { project }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(project: Project) -> Element {
    let coming_class = if project.coming_soon {
        "work-card-coming"
    } else {
        ""
    };

    rsx! {
        div {
            class: "work-card cursor-pointer {coming_class}",
            div {
                class: "work-card-number",
                "{project.number}"
            }
            if project.coming_soon {
                div {
                    class: "coming-soon-badge",
                    "COMING SOON"
                }
            }
            div {
                class: "work-card-inner",
                div {
                    class: "work-card-meta",
                    h3 {
                        class: "work-card-title",
                        "{project.name}"
                    }
                    span {
                        class: "work-card-type",
                        "{project.project_type}"
                    }
                    span {
                        class: "work-card-year",
                        "{project.year}"
                    }
                }
                div {
                    class: "work-card-desc",
                    p {
                        class: "work-card-text",
                        "{project.description}"
                    }
                    div {
                        class: "work-card-tags",
                        for tag in project.tags {
                            span {
                                class: "work-tag",
                                "{tag}"
                            }
                        }
                    }
                }
            }
        }
    }
}
