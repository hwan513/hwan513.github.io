use crate::components::icons::{GitHub, LinkedIn, Pdf};
use leptos::prelude::*;

/// Renders the Hero section
#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero" class="hero">
            <div>
                <code class="terminal-type">"$ infofetch"</code>
                <div class="info">
                    <picture>
                        <source srcset="/images/Henry.webp" type="image/webp" />
                        <img src="/images/Henry.png" alt="Henry Wang" />
                    </picture>
                    <article>
                        <code class="heading">"Henry Wang @ New Zealand"</code>
                        <code>"------------------------"</code>
                        <code>
                            <span>"Occupation: "</span>
                            "Software Engineer"
                        </code>
                        <code>
                            // I have to hard code this man
                            <span>"Uptime: "</span>
                            "22 years, 7 months"
                        </code>
                        <code>
                            <span>"GitHub: "</span>
                            <a href="https://github.com/hwan513" target="_blank">
                                <GitHub />
                                "hwan513"
                            </a>
                        </code>
                        <code>
                            <span>"LinkedIn: "</span>
                            <a href="https://linkedin.com/in/henry-h-wang/" target="_blank">
                                <LinkedIn />
                                "henry-h-wang"
                            </a>
                        </code>
                        <code>
                            <span>"CV: "</span>
                            <a href="/cv.pdf">
                                <Pdf />
                                "/cv.pdf"
                            </a>
                        </code>
                        <code>
                            <span>"Font: "</span>
                            <a href="https://www.recursive.design/" target="_blank">
                                "Recursive"
                            </a>
                        </code>
                        <code>
                            <span>"Host: "</span>
                            <a href="https://github.com/hwan513/hwan513.github.io" target="_blank">
                                "Github Pages"
                            </a>
                        </code>
                        <code>
                            <ul class="color-palette">
                                {(0..6).map(|_| view! { <li /> }).collect_view()}
                            </ul>
                        </code>
                    </article>
                </div>
            </div>
        </section>
    }
}
