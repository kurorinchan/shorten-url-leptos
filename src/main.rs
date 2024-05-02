use leptos::*;
use url::Url;

#[component]
fn App() -> impl IntoView {
    let (url, set_url) = create_signal(String::new());
    let (processed_url, processed_url_set) = create_signal(String::new());
    let (status, set_status) = create_signal(String::new());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        processed_url_set.set("".to_string());
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        let parsed_url = Url::parse(&value);
        let parsed_url = match parsed_url {
            Ok(parsed_url) => parsed_url,
            Err(err) => {
                set_status.set("Failed to parse URL. ".to_string() + &err.to_string());
                return;
            }
        };

        let mut path_iter = parsed_url.path().split('/');

        if let Some(_part) = path_iter.find(|part| part == &"dp") {
            let product_id = path_iter.next();
            let short_url = parsed_url.scheme().to_owned()
                + "://"
                + parsed_url.host_str().unwrap()
                + "/dp/"
                + product_id.unwrap();
            processed_url_set.set(short_url);
            set_status.set("success".to_string());
        } else {
            set_status
                .set("Failed to find product ID. Maybe the URL is not a product URL?".to_string());
        };
    };

    let copy_to_clipboard = move |_url: &str| {
        if let Some(clipboard) = window().navigator().clipboard() {
            // Apparently the result doesn't need to be used.
            clipboard.write_text(&processed_url.get());
            set_status.set("Copied to clipboard!".to_string());
        } else {
            // handle the lack of clipboard!
            set_status.set("Failed to access clipboard".to_string());
        }
    };

    view! {
        <div class="container">
            <h1>"Amazon URL shortener"</h1>
            <div>"Shortens Amazon URLs by removing unnecessary parts of the URL."</div>
            <hr/>
            <form on:submit=on_submit>
                <label for="url">"Enter URL:"</label>
                <input id="url" type="text" autofocus value=url node_ref=input_element/>
                <button type="submit">Submit</button>
            </form>

            <div>"Status: " {move || status.get()}</div>

            {move || {
                if !processed_url.get().is_empty() {
                    view! {
                        <div>
                            <div>
                                <a
                                    href=move || processed_url.get()
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {processed_url.get()}
                                </a>
                            </div>
                            <button on:click=move |_| copy_to_clipboard(
                                &processed_url.get(),
                            )>"Copy to clipboard"</button>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}

        </div>
    }
}

fn main() {
    leptos::mount_to(
        document().body().expect("should have body"),
        move || view! { <App/> },
    )
}
