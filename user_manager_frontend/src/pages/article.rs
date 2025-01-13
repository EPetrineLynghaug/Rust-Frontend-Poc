use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_state, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ArticlePageProps {
    pub slug: String,
}

#[function_component]
pub fn ArticlePage(props: &ArticlePageProps) -> Html {
    let ArticlePageProps { slug } = props;

    let content = use_state(|| "".to_string());

    let slug_clone = slug.clone();
    let content_clone = content.clone();
    spawn_local(async move {
        let res = reqwest::get(format!("https://skby54ey.api.sanity.io/v2022-03-07/data/query/production?query=++*%5B_type+%3D%3D+%22post%22+%26%26+slug.current+%3D%3D+%22{}%22%5D+%7B%0A++++++body%5B%5D+%7B%0A++++...%2C%0A++++asset-%3E%7B...%2C%22_key%22%3A+_id%7D%0A++%7D%2C%0A++name%2C%0A++logo+%7B%0A++++...%2C%0A++++asset-%3E%7B...%2C%22_key%22%3A+_id%7D%0A++%7D%0A++%7D+", slug_clone)).await;

        match res {
            Ok(response) => {
                let text = response.text().await.expect("Badly formatted API response");
                content_clone.set(text.clone());
            }
            Err(err) => {
                log!(&format!("Fetch error: {}", err));
            }
        }
    });

    html! {
        <div>
            <p>{ format!("You are reading article {}", slug) }</p>

            <pre>{ &*content }</pre>
        </div>
    }
}
