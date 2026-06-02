use crate::util::use_scroll_animation;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    use_scroll_animation(".contact-section", ".contact-heading", 1.0, 60.0, 0.0, 0.0);
    use_scroll_animation(".contact-section", ".contact-links", 0.8, 30.0, 0.0, 0.3);

    rsx! {
        section {
            id: "contact",
            class: "contact-section",
            div {
                class: "contact-inner",
                h2 {
                    class: "contact-heading",
                    "READY TO BUILD"
                    br {}
                    "SOMETHING REAL?"
                }
                div {
                    class: "contact-links",
                    a {
                        href: "mailto:hello@tosin.dev",
                        "hello@tosin.dev"
                    }
                    span { class: "text-mist", "·" }
                    a {
                        href: "https://github.com/kagesyntax",
                        target: "_blank",
                        "GitHub"
                    }
                    span { class: "text-mist", "·" }
                    a {
                        href: "https://x.com/kagesyntax",
                        target: "_blank",
                        "X / Twitter"
                    }
                }
                div {
                    class: "contact-divider",
                }
                p {
                    class: "contact-footer",
                    "© 2025 — Built with Rust & Dioxus. Compiled to WebAssembly."
                    br {}
                    "Deployed on Cloudflare Pages."
                }
            }
        }
    }
}
