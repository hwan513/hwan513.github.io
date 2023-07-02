use sycamore::prelude::*;

#[component]
pub fn Contact<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        section (id="contact", style="min-height: 40vh;") {
            h1 { "Contact"}
            p { "There will be a form here where you can send an email to me. In the meantime, feel free to check out my "
                a (href="https://www.linkedin.com/in/henry-h-wang/", target="_blank") {"Linkedin"}
                " or contact me via email at  "
                a (href="mailto:mail@henryh.wang"){ "mail@henryh.wang" }
                "."
            }
        }
    )
}
