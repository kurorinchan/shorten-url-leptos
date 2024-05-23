use leptos::*;
use url::Url;

/// There are 4 ways amazon URL contain product id.
/// 1. <any>/dp/<id>
/// 2. <domain>/gp/product/<id>
/// 3. <domain>/exec/obidos/asin/<id>
/// 4. <domain>/o/ASIN/<id>
/// This function returns the <id> within the above patterns.
fn get_product_id(url: &str) -> Option<String> {
    let url = Url::parse(url).ok()?;

    const DP: &str = "dp";
    const GP: &str = "gp";
    const EXEC: &str = "exec";
    const O: &str = "o";
    let parts = url.path().split('/').collect::<Vec<&str>>();
    for (i, part) in parts.iter().enumerate() {
        if part == &DP {
            if i + 1 >= parts.len() {
                break;
            }
            return Some(parts[i + 1].to_owned());
        }
        if part == &GP {
            if i + 2 >= parts.len() {
                break;
            }
            return Some(parts[i + 2].to_owned());
        }
        if part == &O {
            if i + 2 >= parts.len() {
                break;
            }
            return Some(parts[i + 2].to_owned());
        }

        if part == &EXEC {
            if i + 3 >= parts.len() {
                break;
            }
            return Some(parts[i + 3].to_owned());
        }
    }

    None
}

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

        let product_id = get_product_id(&value);
        let parsed_url = Url::parse(&value);
        let parsed_url = match parsed_url {
            Ok(parsed_url) => parsed_url,
            Err(err) => {
                set_status.set("Failed to parse URL. ".to_string() + &err.to_string());
                return;
            }
        };

        if let Some(product_id) = product_id {
            let short_url = parsed_url.scheme().to_owned()
                + "://"
                + parsed_url.host_str().unwrap()
                + "/dp/"
                + &product_id;
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
                <button
                    type="reset"
                    on:click=move |_| {
                        set_url.set(String::new());
                        input_element.get().unwrap().focus().unwrap();
                    }
                >

                    "Reset"
                </button>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_id_dp() {
        let url = "https://www.amazon.co.jp/dp/B01N9KXH9E";
        assert_eq!(get_product_id(url), Some("B01N9KXH9E".to_string()));
    }

    #[test]
    fn test_get_product_id_dp_with_extra_stuff() {
        let url = "https://www.amazon.co.jp/Comomed-%E3%83%AD%E3%83%BC%E3%83%AB%E8%87%AA%E5%B7%B1%E7%B2%98%E7%9D%80%E5%8C%85%E5%B8%AF%E7%99%BD-5cm-4-5-%E5%BC%BE%E6%80%A7%E5%8C%85%E5%B8%AF%E3%80%81%E4%B8%8D%E7%B9%94%E5%B8%83%E3%80%81%E3%82%B9%E3%83%9D%E3%83%BC%E3%83%84%E3%83%86%E3%83%BC%E3%83%97%E3%80%81%E7%8D%A3%E5%8C%BB%E5%8C%85%E8%A3%85%E3%80%81%E5%8C%85%E5%B8%AF%E5%8C%85%E5%B8%AF%E3%80%81%E6%95%8F%E6%84%9F%E8%82%8C%E3%81%AB%E9%81%A9%E3%81%97%E3%81%A6%E3%81%84%E3%81%BE%E3%81%99%E3%80%82/dp/B07DVHW5QD/ref=sr_1_1_sspa?dib=eyJ2IjoiMSJ9.WJ3pdApniFZc9dR5oLkjieQhitNIeI_JnK-LBk7R1Rcz6Vvza4f8irbDEKCj4lnJaJ6JMKNIQWFqizyTCRUHGipHK6uk1OsTAK43sZMiNj6p-_aYystOXhBEhKt4NfAf9LgAOs-49EW0I0cLqUN7-SqjB_JtLLD2j0oIid2bk97CY16YJOzcR344VXXc-WOwmhaIa9K8fVtQZato6DNaBoHNDsMPKWm1HdbboUofL8pvIW0h2hI3TueyAqusC95WRFU02vgasqxM8_iWORYz1nC4zUfCLwrMsjtpAXQGvHQ.Yo3JEW1BJJOU9FJpDUS2SjigvNbYimbp4284WQmy1qc&dib_tag=se&keywords=%E5%8C%85%E5%B8%AF&qid=1716426773&sr=8-1-spons&sp_csd=d2lkZ2V0TmFtZT1zcF9hdGY&psc=1";
        assert_eq!(get_product_id(url), Some("B07DVHW5QD".to_string()));
    }

    #[test]
    fn test_get_product_id_gp() {
        let url = "https://www.amazon.co.jp/gp/product/B01N9KXH9E";
        assert_eq!(get_product_id(url), Some("B01N9KXH9E".to_string()));
    }

    #[test]
    fn test_get_product_id_gp_with_extra_stuff() {
        let url = "https://www.amazon.co.jp/gp/product/B099J7WNV2/ref=ox_sc_saved_image_2?smid=A22RFVFOGUYRPO&psc=1";
        assert_eq!(get_product_id(url), Some("B099J7WNV2".to_string()));
    }

    #[test]
    fn test_get_product_id_exec() {
        let url = "https://www.amazon.co.jp/exec/obidos/asin/B01N9KXH9E";
        assert_eq!(get_product_id(url), Some("B01N9KXH9E".to_string()));
    }

    #[test]
    fn test_get_product_id_o() {
        let url = "https://www.amazon.co.jp/o/ASIN/B01N9KXH9E";
        assert_eq!(get_product_id(url), Some("B01N9KXH9E".to_string()));
    }
}
