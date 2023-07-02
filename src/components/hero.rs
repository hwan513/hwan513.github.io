use sycamore::prelude::*;

#[component]
pub fn Hero<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        section (id="hero") {
            div (id="heroWrapper"){
                div (id="heroText") {
                    p { "Hi there, I'm" }
                    div (id="heroName") { "Henry Wang"}
                    p { "I am a software engineering student in my penultimate year
                    at the University of Auckland, with a passion to explore 
                    everything that can be offered by software technologies."}
                    div (id="icons") {

                    }
                }
                div (id="heroImage") {
                    img (src="", alt="Photo of Henry Wang")
                }
            }
        }
    )
}
