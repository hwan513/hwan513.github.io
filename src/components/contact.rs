use leptos::prelude::*;

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

    view! {
        <form class="contact-form">
            <fieldset>
                <label>"Name" <input type="text" /></label>
                <label>"Email" <input type="email" /></label>
                <label class="full-width">"Subject" <input type="text" /></label>
                <label class="full-width">
                    "Message"
                    <textarea
                        node_ref=textarea_ref
                        on:input=on_input
                        style=move || format!("height: calc({}px)", message_height())
                    />
                </label>
                <button type="submit">"Send Message"</button>
            </fieldset>
        </form>
    }
}
