use leptos::prelude::*;

/// Renders the About section
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="nav">
            <a href="#">Home</a>
            <a href="#about">About</a>
            <a href="#projects">Projects</a>
            <a href="#contact">Contact</a>
        </nav>
    }
}
