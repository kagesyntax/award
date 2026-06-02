use crate::components;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/style.css");

#[component]
pub fn App() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            function runAppEffects() {
                if (typeof gsap === 'undefined' || typeof ScrollTrigger === 'undefined') {
                    setTimeout(runAppEffects, 100);
                    return;
                }
                gsap.registerPlugin(ScrollTrigger);

                // Scroll progress bar via GSAP
                gsap.to('#scroll-progress', {
                    width: '100%',
                    ease: 'none',
                    scrollTrigger: {
                        trigger: document.body,
                        start: 'top top',
                        end: 'bottom bottom',
                        scrub: 0.3,
                    },
                });
            }
            runAppEffects();
        "#,
        );
    });

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            id: "scroll-progress",
            class: "fixed top-0 left-0 h-2 bg-brand z-60",
            style: "width: 0%;",
        }

        components::Navbar {}
        components::Hero {}
        components::About {}
        components::Stack {}
        components::Work {}
        components::Process {}
        components::Contact {}
    }
}
