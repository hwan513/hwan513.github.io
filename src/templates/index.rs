use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // Better styling will be in the works
        div(style = r#"padding: 2em; font-size: 4vh; max-width: 45ch; font-family: "Inter", "Verdana", "RobotoDraft", "Roboto", sans-serif; line-height: 1.5em"#) {
            p {
                "Hi Hello! I'm Henry Wang, a penultimate year software engineering student at the University of Auckland."
                br {}
                br {}
                "As you can tell, the site here is still very WIP (I'm learning "
                a (href="https://framesurge.sh/perseus") {"Perseus"}
                " to create this website). In the meantime, feel free to check out my "
                a (href="https://www.linkedin.com/in/henry-h-wang/") {"Linkedin"}
                " or contact me via email at  "
                a (href="mailto:wang.henryhan@gmail.com"){ "wang.henryhan@gmail.com" }
                "."
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Henry Wang" }
        link (
          href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap",
          rel="stylesheet"
        ) {}
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
