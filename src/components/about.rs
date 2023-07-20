use sycamore::prelude::*;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        section(
            id = "about",
            dangerously_set_inner_html =
                &markdown::to_html(include_str!("../../markdown/about.md"))
        )
    )
}
