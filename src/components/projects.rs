use leptos::prelude::*;

/// This is so cursed I swear
macro_rules! project {
    ($project_id:literal, $md_path:literal, $img_path:literal, $alt:literal) => {
        view! {
            <div class="project" id=$project_id>
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
                "blurple", "../markdown/blurple.md", "images/blurple", "Blurple Canvas main screen"
            )}
            {project!(
                "leptos_tech_demo", "../markdown/leptos.md", "images/leptos_demo", "Leptos tech demo pokedex page"
            )}
            {project!(
                "typefaceoff", "../markdown/typefaceoff.md", "images/typefaceoff", "Typefaceoff home page"
            )}
            {project!(
                "quickDraw", "../markdown/quick_draw.md", "images/quickDraw1", "Quick Draw! game mid round"
            )}
        </section>
    }
}
