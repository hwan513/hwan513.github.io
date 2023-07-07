use std::rc::Rc;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use sycamore::{futures::spawn_local_scoped, prelude::*, rt::Event};

#[cfg(client)]
use web_sys::window;

enum FormStatus {}

#[component]
pub fn Contact<G: Html>(cx: Scope) -> View<G> {
    let form_complete = create_signal(cx, false);
    view!(cx,
        section (id="contact", style="min-height: 40vh;") {
            h1 { "Contact Me 👋"}
            p { "Fill the form to send me an email." }
            ContactForm (form_complete=form_complete)
        }
    )
}

#[derive(Serialize, Deserialize)]
struct ContactDetails {
    access_key: String,
    name: Rc<String>,
    email: Rc<String>,
    subject: Rc<String>,
    message: Rc<String>,
}

impl ContactDetails {
    fn new(name: Rc<String>, email: Rc<String>, subject: Rc<String>, message: Rc<String>) -> Self {
        Self {
            access_key: String::from("ee4bf239-f98e-42e2-ac58-aba20511b885"),
            name,
            email,
            subject,
            message,
        }
    }
}

#[component(inline_props)]
fn ContactForm<'a, G: Html>(cx: Scope<'a>, form_complete: &'a Signal<bool>) -> View<G> {
    let botcheck = create_signal(cx, false);
    let name = create_signal(cx, String::new());
    let email = create_signal(cx, String::new());
    let subject = create_signal(cx, String::new());
    let mesage = create_signal(cx, String::new());
    let submit_handler = move |e: Event| {
        spawn_local_scoped(cx, async move {
            e.prevent_default();
            if *botcheck.get() {
                return;
            }

            let contact_details =
                ContactDetails::new(name.get(), email.get(), subject.get(), mesage.get());

            let json = serde_json::to_string(&contact_details).unwrap();

            let client = reqwest::Client::new();
            // TODO: add styling for form handling
            let _res = client
                .post("https://api.web3forms.com/submit")
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .body(json)
                .send()
                .await
                .unwrap();
            form_complete.set(true);
        })
    };

    view!(cx,
        fieldset () {
            form (on:submit=submit_handler, id="contactForm", action="https://api.web3forms.com/submit",  method="POST"){

                input(type="hidden", name="access_key", value="ee4bf239-f98e-42e2-ac58-aba20511b885")
                input(bind:checked=botcheck, type="checkbox", name="botcheck", id="", style="display:none")

                div (class="contact-field half") {
                    label (for="contactName"){ "Full Name" }
                    input (bind:value=name, type="text", name="name", id="contactName", required=true)
                }

                div (class="contact-field half") {
                    label (for="contactEmail"){ "Email" }
                    input (bind:value=email, type="email", name="email", id="contactEmail", required=true)
                }

                div (class="contact-field full") {
                    label (for="contactSubject"){ "Subject" }
                    input (bind:value=subject, type="text", name="subject", id="contactSubject", required=true)
                }

                ContactMessage(bind_value=mesage)

                button (type="submit", class="contact-field full") { "Send Message" }
            }
        }
    )
}

#[component(inline_props)]
fn ContactMessage<'a, G: Html>(cx: Scope<'a>, bind_value: &'a Signal<String>) -> View<G> {
    let message_height = create_signal(cx, 0);

    // TODO: see if I can actually use the node_ref
    // let node_ref = create_node_ref(cx);

    let input_handler = move |_| {
        #[cfg(client)]
        {
            let document = window().unwrap().document().unwrap();
            let contact_message = document.get_element_by_id("contactMessage").unwrap();
            message_height.set(0);
            message_height.set(contact_message.scroll_height());
        }
    };

    view!(cx,
        div(class="contact-field full") {
            label(for="contactMessage") { "Message" }
            textarea(
                // ref=node_ref,
                bind:value=bind_value,
                on:input=input_handler,
                style=format!("height: calc({}px - 1em)", message_height.get()),
                name="contactMessage",
                id="contactMessage",
                required=true)
        }
    )
}
