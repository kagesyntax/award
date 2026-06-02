use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            function runNavbarEffect() {
                if (typeof gsap === 'undefined' || typeof ScrollTrigger === 'undefined') {
                    setTimeout(runNavbarEffect, 100);
                    return;
                }
                gsap.registerPlugin(ScrollTrigger);
                const navbar = document.getElementById('navbar');
                ScrollTrigger.create({
                    trigger: document.body,
                    start: '80px top',
                    onEnter: () => {
                        navbar.style.backgroundColor = 'rgba(8, 8, 8, 0.95)';
                        navbar.style.backdropFilter = 'blur(8px)';
                    },
                    onLeaveBack: () => {
                        navbar.style.backgroundColor = 'transparent';
                        navbar.style.backdropFilter = 'none';
                    },
                });
            }
            runNavbarEffect();
        "#,
        );
    });

    let scroll_to = |id: &str| {
        let id = id.to_string();
        move |_| {
            dioxus::document::eval(&format!(
                "document.getElementById('{}').scrollIntoView({{behavior: 'smooth'}});",
                id
            ));
        }
    };

    rsx! {
        nav {
            id: "navbar",
            class: "navbar",
            div {
                class: "navbar-inner",
                div {
                    class: "navbar-brand",
                    "TOSIN"
                }
                div {
                    class: "navbar-links",
                    a {
                        class: "navbar-link",
                        onclick: scroll_to("hero"),
                        "HOME"
                    }
                    a {
                        class: "navbar-link",
                        onclick: scroll_to("work"),
                        "WORK"
                    }
                    a {
                        class: "navbar-link",
                        onclick: scroll_to("stack"),
                        "STACK"
                    }
                    a {
                        class: "navbar-link",
                        onclick: scroll_to("process"),
                        "PROCESS"
                    }
                    a {
                        class: "navbar-link",
                        onclick: scroll_to("contact"),
                        "CONTACT"
                    }
                }
            }
        }
    }
}
