use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            function runContactAnimation() {
                if (typeof gsap === 'undefined' || typeof ScrollTrigger === 'undefined') {
                    setTimeout(runContactAnimation, 100);
                    return;
                }
                gsap.registerPlugin(ScrollTrigger);
                console.log('GSAP ScrollTrigger loaded, running contact animation');
                gsap.from('.contact-heading', {
                    scrollTrigger: { trigger: '.contact-section', start: 'top 75%' },
                    duration: 1.0, y: 60, ease: 'power3.out'
                });
                gsap.from('.contact-links', {
                    scrollTrigger: { trigger: '.contact-section', start: 'top 75%' },
                    duration: 0.8, y: 30, ease: 'power2.out', delay: 0.3
                });
            }
            runContactAnimation();
        "#,
        );
    });

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
                        href: "mailto:hello@yourdomain.com",
                        "hello@yourdomain.com"
                    }
                    span { class: "text-mist", "·" }
                    a {
                        href: "https://github.com",
                        target: "_blank",
                        "GitHub"
                    }
                    span { class: "text-mist", "·" }
                    a {
                        href: "https://twitter.com",
                        target: "_blank",
                        "Twitter/X"
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
