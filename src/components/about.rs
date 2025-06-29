use leptos::prelude::*;

/// Renders the About section
#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="about">
            <picture>
                <source srcset="/images/painting.webp" type="image/webp" />
                <img src="/images/painting.png" alt="Painting of bird, some fish, and a sunset" />
            </picture>
            <article inner_html=markdown::to_html(include_str!("../markdown/about.md"))></article>
        </section>
    }
}
