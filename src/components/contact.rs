use sycamore::prelude::*;

#[cfg(client)]
use web_sys::window;

#[component]
pub fn Contact<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        section (id="contact", style="min-height: 40vh;") {
            h1 { "Contact Me 👋"}
            p { "Fill the form to send me an email." }
            form (id="contactForm"){
                ContactForm(FieldProps::new("Full Name", "text", "contactName", "half"))
                ContactForm(FieldProps::new("Email", "email", "contactEmail","half"))
                ContactForm(FieldProps::new("Subject", "text", "contactSubject", "full"))
                ContactMessage
                ContactForm(FieldProps::new("", "submit", "Send", "full"))
            }
        }
    )
}

#[derive(Prop)]
struct FieldProps {
    field_label: String,
    field_type: String,
    field_name: String,
    field_class: String,
}

impl FieldProps {
    fn new(field_label: &str, field_type: &str, field_name: &str, field_class: &str) -> Self {
        Self {
            field_label: field_label.to_string(),
            field_type: field_type.to_string(),
            field_name: field_name.to_string(),
            field_class: field_class.to_string(),
        }
    }
}

#[component]
fn ContactForm<G: Html>(cx: Scope, props: FieldProps) -> View<G> {
    let FieldProps {
        field_label,
        field_type,
        field_name,
        field_class,
    } = props;
    let field_name_clone = field_name.clone();
    let field_name_clone_clone = field_name.clone();

    view!(cx,
        div(class=(format!("contact-field {field_class}"))) {
            label (for=field_name){ (field_label) }
            input (type=field_type, name=field_name_clone, id=field_name_clone_clone, required=true)
        }
    )
}

#[component]
fn ContactMessage<G: Html>(cx: Scope) -> View<G> {
    let message_height = create_signal(cx, 0);

    // TODO: see if I can actually use the node_ref
    // let node_ref = create_node_ref(cx);

    let input_handler = move |_| {
        #[cfg(client)]
        {
            let document = window().unwrap().document().unwrap();
            let contact_message = document.get_element_by_id("contactMessage").unwrap();
            contact_message.scroll_height();
            message_height.set(0);
            message_height.set(contact_message.scroll_height());
        }
    };

    view!(cx,
        div(class="contact-field") {
            label(for="contactMessage") { "Message" }
            textarea(
                // ref=node_ref,
                on:input=input_handler,
                style=format!("height: calc({}px - 1em)", message_height.get()),
                name="contactMessage",
                id="contactMessage",
                required=true)
        }
    )
}
