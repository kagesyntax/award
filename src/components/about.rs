use crate::util::use_scroll_animation;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    use_scroll_animation(".about-section", ".about-copy", 0.9, 50.0, 0.0, 0.0);
    use_scroll_animation(".about-section", ".about-number", 0.6, 30.0, 0.0, 0.0);

    rsx! {
        section {
            id: "about",
            class: "about-section",
            div {
                class: "max-w-content mx-auto",
                div {
                    class: "about-grid",
                    div {
                        class: "about-label-col",
                        div {
                            class: "section-number",
                            "01"
                        }
                        span {
                            class: "section-label",
                            "ABOUT"
                        }
                    }
                    div {
                        class: "about-copy font-outfit text-text",
                        p { class: "mb-8 text-3xl leading-relaxed",
                            "I started writing Rust because I wanted to build things that didn't apologize for being fast. Dioxus gave me a way to bring that philosophy to the web — full-stack, compiled, real. No runtime bloat. No compromise."
                        }
                        p { class: "text-3xl leading-relaxed",
                            "I'm based in Nigeria. I work with designers and founders who want to ship something that actually looks and performs like it cost what it should. Every project starts with the same question: what's the fastest path from this idea to something real?"
                        }
                    }
                }
            }
        }
    }
}
