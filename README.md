# Henry Wang's personal website

This is the repo to my portfolio website, build leveraging [Leptos's](https://leptos.dev) static site generator.
I made a few small adjustments to the build default axum ssr template to make it so that `cargo serve --release`
generates a static site in the `dist` folder. Majority of commands and tools are being handled by
[mise](https://mise.jdx.dev). The more important scripts include:

```sh
mise run build:release  # to build the site in release mode
mise run format         # to check formatting of the code
mise run lint           # to check linting the code
mise run dev            # to run the local dev server at localhost:3000
```

To check for other defined tasks run `mise tasks`
