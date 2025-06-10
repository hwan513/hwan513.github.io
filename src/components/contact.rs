use std::collections::HashMap;

use leptos::{ev::SubmitEvent, leptos_dom::logging::console_log, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::components::Plane;

/// Renders the Contact form section
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section>
            <h1>"Contact me 👋"</h1>
            <p>"My details are in my CV, but you can still reach me from here."</p>
            <ContactForm />
        </section>
    }
}

async fn submit_form(params: Vec<(String, String)>) {
    console_log(&format!("Form data: {params:?}"));
    let map = params
        .iter()
        .map(|(k, v)| (k, v))
        .collect::<HashMap<_, _>>();
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.web3forms.com/submit")
        .json(&map)
        .send()
        .await;
    match res {
        Ok(response) => {
            console_log(&format!("Response: {response:?}"));
        }
        Err(err) => {
            console_log(&format!("Error: {err:?}"));
        }
    }
}

/// Contact form implementation, which hands the form handling to Web3Forms
#[component]
fn ContactForm() -> impl IntoView {
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();
    let (message_height, set_message_height) = signal(0);
    let on_input = move |_| {
        if let Some(textarea) = textarea_ref.get() {
            // Setting it to 0 first ensures that the textarea can decrease in height
            set_message_height(0);
            // Bit of a hack here, leptos renders too quickly for the textarea height adjustment reduce correctly
            // Wrap it in a use effect to ensure that the height is set next cycle
            Effect::new(move |_| {
                set_message_height(textarea.scroll_height());
            });
        }
    };

    // Submit contact form data to Web3Forms
    let on_submit = |ev: SubmitEvent| {
        ev.prevent_default();
        let form = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlFormElement>().ok())
            .expect("Event target is not a form");
        let form_data = FormData::new_with_form(&form).expect("FormData creation failed");
        let form_to_string =
            |attribute: &str| -> String { form_data.get(attribute).as_string().unwrap() };
        let params = [
            (
                "access_key".to_string(),
                "ee4bf239-f98e-42e2-ac58-aba20511b885".to_string(),
            ),
            ("name".to_string(), form_to_string("name")),
            ("email".to_string(), form_to_string("email")),
            ("subject".to_string(), form_to_string("subject")),
            ("message".to_string(), form_to_string("message")),
            // ( "botcheck".to_string(), form_data.get("botcheck").as_bool().unwrap().to_string(),),
            // TODO: Potentially add honeypot to prevent bot spam.
        ];
        let _ = LocalResource::new(move || submit_form(params.to_vec()));
    };

    view! {
        <form class="contact-form" on:submit=on_submit>
            <fieldset>
                <label>"Name" <input type="text" name="name" /></label>
                <label>"Email" <input type="email" name="email" /></label>
                <label class="full-width">"Subject" <input type="text" name="subject" /></label>
                <label class="full-width">
                    "Message"
                    <textarea
                        node_ref=textarea_ref
                        on:input=on_input
                        name="message"
                        style=move || format!("height: calc({}px)", message_height())
                    />
                </label>
                <input type="checkbox" name="botcheck" class="hidden" style="display: none;" />
                <button type="submit">"Send" <Plane /></button>
            </fieldset>
        </form>
    }
}
