use leptos::prelude::*;

/// This is so cursed I swear
macro_rules! project {
    ($md_path:literal, $img_path:literal, $alt:literal) => {
        view! {
            <div class="project">
                <picture>
                    <source srcset=concat!($img_path, ".webp") type="image/webp" />
                    <img src=concat!($img_path, ".png") alt=$alt />
                </picture>
                <article inner_html=markdown::to_html(include_str!($md_path))></article>
            </div>
        }
    };
}

/// Renders the About section
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="projects">
            <h1>"Projects 💼"</h1>
            {project!(
                "../markdown/quick_draw.md", "../images/quick_draw", "Quick Draw! game mid round"
            )}
        </section>
    }
}
