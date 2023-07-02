use sycamore::prelude::*;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        section (id="about", style="min-height: 40vh;") {
            h1 { "About me"}
            p { "Usually, I would mention some information about me, however, as you can see here,
                the site here is still very WIP (I'm using Perseus to create this website)."}
        }
    )
}
