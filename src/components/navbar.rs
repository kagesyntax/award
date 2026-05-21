use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            const navbar = document.getElementById('navbar');
            window.addEventListener('scroll', () => {
                if (window.scrollY > 80) {
                    navbar.style.backgroundColor = 'rgba(8, 8, 8, 0.95)';
                    navbar.style.backdropFilter = 'blur(8px)';
                } else {
                    navbar.style.backgroundColor = 'transparent';
                    navbar.style.backdropFilter = 'none';
                }
            });
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
                    "ADEDEJI"
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
