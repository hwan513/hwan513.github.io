use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // Better styling will be in the works

        main {
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

        // general stylesheets
        link (href="https://fonts.googleapis.com/css2?family=Recursive:wght,CASL,CRSV@300..800,0..1,0..1&display=swap", rel="stylesheet")
        link (href=".perseus/static/reset.css", rel="stylesheet")
        link (href=".perseus/static/index.css", rel="stylesheet")
        link (href=".perseus/static/colors.css", rel="stylesheet")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
