#![recursion_limit = "256"]
#[cfg(feature = "ssr")]
#[tokio::main]
#[allow(unused_imports, unused_variables)]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list_with_ssg};
    use portfolio::app::shell;

    let get_configuration = get_configuration(None).unwrap();
    let conf = get_configuration;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });
    // Generate static routes to be served in `dist/`
    static_routes.generate(&leptos_options).await;

    // Only run the axum server in debug mode
    // when it is expected to be run using `cargo leptos watch`
    #[cfg(debug_assertions)]
    {
        let app = Router::new()
            .leptos_routes(&leptos_options, routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        // Run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}
