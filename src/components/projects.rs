use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[component]
pub fn Projects<G: Html>(cx: Scope) -> View<G> {
    let projects = include_str!("../../projects/data.json");
    let projects: Vec<ProjectProps> = serde_json::from_str(projects).unwrap();
    let thing = projects[0].clone();
    view!(cx,
        section (id="projects") {
            h1 { "Projects"}
            Project (thing)
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
    view!(cx,
        article (class="project", id=props.id) {
            div (class="project_text") {
                h2 { (props.name) }
                div (class="project_technologies") { (props.technologies.join(", ")) }
                p { (props.description) }

                (if props.repo.is_empty() {
                    view!(cx,
                        a (href=link){  })
                } else {view!(cx,)})
            }
            div (class="project_image") {
                //
            }

        }
    )
}
