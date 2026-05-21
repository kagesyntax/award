use crate::components;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/style.css");

#[component]
pub fn App() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            const progressBar = document.getElementById('scroll-progress');
            window.addEventListener('scroll', () => {
                const scrollTop = window.scrollY;
                const docHeight = document.documentElement.scrollHeight - window.innerHeight;
                const scrollPercent = (scrollTop / docHeight) * 100;
                progressBar.style.width = scrollPercent + '%';
            });

            // Custom cursor
            const cursor = document.createElement('div');
            cursor.className = 'custom-cursor';
            document.body.appendChild(cursor);

            const dot = document.createElement('div');
            dot.className = 'custom-cursor-dot';
            document.body.appendChild(dot);

            let mouseX = 0, mouseY = 0;
            let cursorX = 0, cursorY = 0;

            document.addEventListener('mousemove', (e) => {
                mouseX = e.clientX;
                mouseY = e.clientY;
                dot.style.left = mouseX + 'px';
                dot.style.top = mouseY + 'px';
            });

            function animateCursor() {
                cursorX += (mouseX - cursorX) * 0.15;
                cursorY += (mouseY - cursorY) * 0.15;
                cursor.style.left = cursorX + 'px';
                cursor.style.top = cursorY + 'px';
                requestAnimationFrame(animateCursor);
            }
            animateCursor();

            const hoverElements = document.querySelectorAll('a, button, .cursor-pointer, .stack-card, .hero-button');
            hoverElements.forEach(el => {
                el.addEventListener('mouseenter', () => cursor.classList.add('cursor-hover'));
                el.addEventListener('mouseleave', () => cursor.classList.remove('cursor-hover'));
            });

            // Re-observe for dynamically added elements
            const observer = new MutationObserver(() => {
                document.querySelectorAll('a, button, .cursor-pointer, .stack-card, .hero-button').forEach(el => {
                    if (!el.dataset.cursorBound) {
                        el.dataset.cursorBound = 'true';
                        el.addEventListener('mouseenter', () => cursor.classList.add('cursor-hover'));
                        el.addEventListener('mouseleave', () => cursor.classList.remove('cursor-hover'));
                    }
                });
            });
            observer.observe(document.body, { childList: true, subtree: true });
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
