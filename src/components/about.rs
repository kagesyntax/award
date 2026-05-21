use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            function runAboutAnimation() {
                if (typeof gsap === 'undefined' || typeof ScrollTrigger === 'undefined') {
                    setTimeout(runAboutAnimation, 100);
                    return;
                }
                gsap.registerPlugin(ScrollTrigger);
                console.log('GSAP ScrollTrigger loaded, running about animation');
                gsap.from('.about-copy', {
                    scrollTrigger: { trigger: '.about-section', start: 'top 75%' },
                    duration: 0.9, y: 50, ease: 'power3.out'
                });
                gsap.from('.about-number', {
                    scrollTrigger: { trigger: '.about-section', start: 'top 75%' },
                    duration: 0.6, y: 30, ease: 'power2.out'
                });
            }
            runAboutAnimation();
        "#,
        );
    });

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
                        class: "about-copy font-instrument-serif text-text",
                        p { class: "mb-8 text-3xl leading-relaxed",
                            "I started writing Rust because I wanted to build things that didn't apologize for being fast. Dioxus gave me a way to bring that philosophy to the web — full-stack, compiled, real. No runtime bloat. No compromise."
                        }
                        p { class: "text-3xl leading-relaxed",
                            "I'm based in Nigeria. I work with designers and founders who want to ship something that actually looks and performs like it cost what it should. One month in. Already not going back."
                        }
                    }
                }
            }
        }
    }
}
