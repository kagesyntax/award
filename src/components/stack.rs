use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::*;
use dioxus_free_icons::Icon;

#[derive(Clone)]
struct StackItem {
    name: &'static str,
    description: &'static str,
    icon_fn: fn() -> Element,
    category: &'static str,
}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.description == other.description
            && self.category == other.category
    }
}

#[component]
pub fn Stack() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            function runStackAnimation() {
                if (typeof gsap === 'undefined' || typeof ScrollTrigger === 'undefined') {
                    setTimeout(runStackAnimation, 100);
                    return;
                }
                gsap.registerPlugin(ScrollTrigger);
                gsap.from('.stack-card', {
                    scrollTrigger: {
                        trigger: '.stack-section',
                        start: 'top 75%',
                    },
                    duration: 0.8,
                    y: 40,
                    stagger: 0.1,
                    ease: 'power3.out'
                });
            }
            runStackAnimation();
        "#,
        );
    });

    let stack_items = vec![
        StackItem {
            name: "Rust",
            description: "The bedrock. Memory safety without the garbage collector.",
            icon_fn: || rsx! { Icon { icon: LdCpu, width: 32, height: 32 } },
            category: "CORE",
        },
        StackItem {
            name: "Dioxus",
            description: "Fullstack Rust UI. Fast, type-safe, and truly cross-platform.",
            icon_fn: || rsx! { Icon { icon: LdCode, width: 32, height: 32 } },
            category: "WEB",
        },
        StackItem {
            name: "WebAssembly",
            description: "Near-native performance in the browser via LLVM.",
            icon_fn: || rsx! { Icon { icon: LdZap, width: 32, height: 32 } },
            category: "WEB",
        },
        StackItem {
            name: "Axum",
            description: "Ergonomic and modular web framework for the Rust ecosystem.",
            icon_fn: || rsx! { Icon { icon: LdServer, width: 32, height: 32 } },
            category: "BACKEND",
        },
        StackItem {
            name: "PostgreSQL",
            description: "The reliable foundation for complex relational data structures.",
            icon_fn: || rsx! { Icon { icon: LdDatabase, width: 32, height: 32 } },
            category: "DATA",
        },
        StackItem {
            name: "TailwindCSS",
            description: "Utility-first CSS for rapid, maintainable design systems.",
            icon_fn: || rsx! { Icon { icon: LdPalette, width: 32, height: 32 } },
            category: "DESIGN",
        },
        StackItem {
            name: "Tokio",
            description: "The industry-standard asynchronous runtime for Rust.",
            icon_fn: || rsx! { Icon { icon: LdActivity, width: 32, height: 32 } },
            category: "CORE",
        },
        StackItem {
            name: "Turso / SQLite",
            description: "Edge-first data distribution for low-latency global apps.",
            icon_fn: || rsx! { Icon { icon: LdHardDrive, width: 32, height: 32 } },
            category: "DATA",
        },
    ];

    rsx! {
        section {
            id: "stack",
            class: "stack-section",
            div {
                class: "max-w-content mx-auto",
                div {
                    class: "section-header",
                    div {
                        class: "section-number",
                        "02"
                    }
                    div {
                        class: "flex flex-col",
                        span {
                            class: "section-label",
                            "TECH STACK"
                        }
                        h2 {
                            class: "font-syne text-3xl font-bold text-text mt-2",
                            "Tools of the trade."
                        }
                    }
                }
                div {
                    class: "stack-grid",
                    for item in stack_items {
                        StackCard {
                            key: "{item.name}",
                            item: item
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StackCard(item: StackItem) -> Element {
    rsx! {
        div {
            class: "stack-card group",
            div {
                class: "flex justify-between items-start mb-6",
                div {
                    class: "stack-card-icon text-brand opacity-60 group-hover:opacity-100 transition-opacity",
                    {(item.icon_fn)()}
                }
                span {
                    class: "font-dm-mono text-[10px] tracking-widest text-sub border border-border px-2 py-1 rounded",
                    "{item.category}"
                }
            }
            h3 {
                class: "stack-card-title group-hover:text-brand transition-colors",
                "{item.name}"
            }
            p {
                class: "stack-card-desc",
                "{item.description}"
            }
        }
    }
}
