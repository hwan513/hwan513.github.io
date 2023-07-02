use sycamore::prelude::*;

#[component]
pub fn Projects<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        section (id="projects", style="min-height: 40vh;") {
            h1 { "Projects"}
            p { "My projects will be showcased here. Look forward to seeing them."
            }
        }
    )
}
