use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[component]
pub fn Projects<G: Html>(cx: Scope) -> View<G> {
    let projects = include_str!("../../projects/data.json");
    let projects: Vec<ProjectProps> = serde_json::from_str(projects).unwrap();
    let projects_view = View::new_fragment(
        projects
            .into_iter()
            .map(|project| view!(cx, Project(project)))
            .collect(),
    );

    view!(cx,
        section (id="projects") {
            h1 { "Projects"}
            (projects_view)
            div (class="project")
        }
    )
}

#[derive(Prop, Clone, Serialize, Deserialize)]
struct ProjectProps {
    id: String,
    name: String,
    technologies: Vec<String>,
    description: String,
    repo: String,
    image: String,
    alt_text: String,
}

#[component]
fn Project<G: Html>(cx: Scope, props: ProjectProps) -> View<G> {
    let link = create_ref(cx, props.repo.clone());
    let alt_text = create_ref(cx, props.alt_text.clone());
    let image_dir = ".perseus/static/images/projects/";
    let image_webp = create_ref(cx, format!("{image_dir}{}.webp", props.image));
    let image_jpeg = create_ref(cx, format!("{image_dir}{}.jpeg", props.image));
    view!(cx,
        article (class="project", id=props.id) {
            div (class="project-text") {
                h2 { (props.name) }
                div (class="project-technologies") { (props.technologies.join(", ")) }
                p { (props.description) }

                (if !props.repo.is_empty() {
                    view!(cx,
                        a (href=link){ "Check out the source code on GitHub" })
                } else {view!(cx,)})
            }
            (if !props.image.is_empty() {
                view!(cx,
                    div (class="project-image") {
                            picture {
                                source (srcset=image_webp)
                                img (src=image_jpeg, alt=alt_text)
                            }
                    })
            } else {view!(cx,)})

        }
    )
}
