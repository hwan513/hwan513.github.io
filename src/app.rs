use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    SsrMode, StaticSegment,
    components::{Route, Router, Routes},
    static_routes::StaticRoute,
};

use crate::pages::home::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Entry point for the web application
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // Injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/portfolio.css" />
        // sets the document title
        <Title text="Henry Wang" />

        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route
                    path=StaticSegment("")
                    view=HomePage
                    ssr=SsrMode::Static(StaticRoute::new())
                />
            </Routes>
        </Router>
    }
}
