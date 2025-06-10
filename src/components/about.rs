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
                <img src="/assets/me.jpg" alt="Picture of me" />
            </picture>
        </section>
    }
}
