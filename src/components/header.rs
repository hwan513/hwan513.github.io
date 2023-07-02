use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view!(cx, header { NavLinks })
}

#[component]
fn NavLinks<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        nav {
            ul {
                li { a(href="#") {"Home"} }
                li { a(href="#about") {"About Me"} }
                li { a(href="#projects") {"Projects"} }
                li { a(href="#contact") {"Contact"} }
            }
        }
    )
}
