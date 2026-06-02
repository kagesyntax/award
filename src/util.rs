use dioxus::prelude::*;

pub fn use_scroll_animation(
    trigger: &'static str,
    target: &'static str,
    duration: f64,
    y: f64,
    stagger: f64,
    delay: f64,
) {
    use_effect(move || {
        dioxus::document::eval(&format!(
            r#"(function() {{
                function run() {{
                    if (typeof gsap === 'undefined' || typeof ScrollTrigger === 'undefined') {{
                        setTimeout(run, 100);
                        return;
                    }}
                    gsap.registerPlugin(ScrollTrigger);
                    gsap.from('{target}', {{
                        scrollTrigger: {{ trigger: '{trigger}', start: 'top 75%' }},
                        duration: {duration},
                        y: {y},
                        ease: 'power3.out',
                        stagger: {stagger},
                        delay: {delay},
                    }});
                }}
                run();
            }})();"#,
        ));
    });
}
