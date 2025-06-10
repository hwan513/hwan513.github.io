use leptos::prelude::*;

use crate::components::{About, Contact, Hero, Nav, Projects};

/// Renders the homepage
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Nav />
        <main>
            <Hero />
            <About />
            <Projects />
            <Contact />
        </main>
    }
}
