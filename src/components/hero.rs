use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    use_effect(move || {
        dioxus::document::eval(
            r#"
            function runHeroAnimation() {
                if (typeof gsap === 'undefined') {
                    setTimeout(runHeroAnimation, 100);
                    return;
                }
                console.log('GSAP loaded, running hero animation');
                gsap.from('.hero-line-1', { duration: 1.0, y: 80, ease: 'power4.out' });
                gsap.from('.hero-line-2', { duration: 1.0, y: 80, ease: 'power4.out', delay: 0.1 });
                gsap.from('.hero-line-3', { duration: 1.0, y: 80, ease: 'power4.out', delay: 0.2 });
                gsap.from('.hero-sub', { duration: 0.8, ease: 'power2.out', delay: 0.5 });
                gsap.from('.hero-cta', { duration: 0.8, y: 20, ease: 'power2.out', delay: 0.7 });
                gsap.from('.hero-terminal', { duration: 1.2, x: 40, ease: 'power3.out', delay: 0.3 });
            }
            runHeroAnimation();
        "#,
        );
    });

    let terminal_code = r#"<span style="color: #C586C0;">fn</span> <span style="color: #DCDCAA;">main</span>() {
    <span style="color: #DCDCAA;">launch</span>(<span style="color: #4EC9B0;">App</span>);
}

<span style="color: #C586C0;">#[</span><span style="color: #4EC9B0;">component</span><span style="color: #C586C0;">]</span>
<span style="color: #C586C0;">fn</span> <span style="color: #DCDCAA;">App</span>() -> <span style="color: #4EC9B0;">Element</span> {
    <span style="color: #C586C0;">let</span> signal = <span style="color: #DCDCAA;">use_signal</span>(|| <span style="color: #B5CEA8;">0</span>);
    <span style="color: #DCDCAA;">rsx!</span> {
        <span style="color: #569CD6;">div</span> { <span style="color: #9CDCFE;">class</span>: <span style="color: #CE9178;">"container"</span>,
            <span style="color: #569CD6;">h1</span> { <span style="color: #CE9178;">"Built in Rust."</span> }
            <span style="color: #569CD6;">p</span> { <span style="color: #CE9178;">"Fast by default."</span> }
        }
    }
}"#;

    rsx! {
        section {
            id: "hero",
            class: "hero-section",
            div {
                class: "hero-inner",
                div {
                    class: "hero-content",
                    div {
                        class: "hero-text",
                        h1 {
                            class: "font-syne font-extrabold text-text tracking-tight hero-headline",
                            div { class: "hero-line-1", "I BUILD" }
                            div { class: "hero-line-2", "THINGS THAT" }
                            div { class: "hero-line-3", "PERFORM." }
                        }
                        p {
                            class: "hero-sub font-instrument-serif font-italic text-sub",
                            "Rust developer. Dioxus specialist.\nObsessed with speed and precision."
                        }
                        div {
                            class: "hero-cta",
                            a {
                                class: "hero-button",
                                onclick: move |_| {
                                    dioxus::document::eval("document.getElementById('work').scrollIntoView({behavior: 'smooth'});");
                                },
                                "SEE MY WORK ↓"
                            }
                        }
                    }
                    div {
                        class: "hero-terminal",
                        div {
                            class: "terminal-card",
                            div {
                                class: "terminal-header",
                                div { class: "terminal-dot-red" }
                                div { class: "terminal-dot-yellow" }
                                div { class: "terminal-dot-green" }
                                span {
                                    class: "font-dm-mono text-sub",
                                    "main.rs"
                                }
                            }
                            pre {
                                class: "terminal-code",
                                code {
                                    dangerous_inner_html: terminal_code,
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "ticker-wrapper",
                div {
                    class: "ticker-animate ticker-text",
                    {
                        let ticker = "RUST · DIOXUS · WEBASSEMBLY · PERFORMANCE · TAILWIND · PRECISION · ";
                        rsx! {
                            span { "{ticker}{ticker}{ticker}{ticker}{ticker}{ticker}" }
                            span { "{ticker}{ticker}{ticker}{ticker}{ticker}{ticker}" }
                        }
                    }
                }
            }
        }
    }
}
