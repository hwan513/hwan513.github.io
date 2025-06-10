use leptos::prelude::*;

/// Renders the About section
#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about">
            <article>
                <h1>"About me 👨‍💻"</h1>
            </article>
            <picture>
                <img src="/images/painting.jpg" alt="Painting of bird, some fish, and a sunset" />
            </picture>
        </section>
    }
}
