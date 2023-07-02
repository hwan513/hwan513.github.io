use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::{
    about::About, contact::Contact, header::Header, hero::Hero, projects::Projects,
};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header
        main {
            Hero
            About
            Projects
            Contact
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Henry Wang" }

        // general stylesheets
        link (href="https://fonts.googleapis.com/css2?family=Recursive:wght,CASL,CRSV@300..800,0..1,0..1&display=swap", rel="stylesheet")
        link (href=".perseus/static/css/reset.css", rel="stylesheet")
        link (href=".perseus/static/css/index.css", rel="stylesheet")
        link (href=".perseus/static/css/colors.css", rel="stylesheet")

        // component stylesheet
        link ( href=".perseus/static/css/header.css", rel="stylesheet")
        link ( href=".perseus/static/css/hero.css", rel="stylesheet")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
