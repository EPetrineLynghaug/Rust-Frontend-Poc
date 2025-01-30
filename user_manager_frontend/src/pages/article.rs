use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]

// This struct defines the props (properties) our `ArticlePage` component will receive.
// In this case, it only needs a `slug` to identify which article to load.
pub struct ArticlePageProps {
    pub slug: String,
}
// This struct matches the top-level JSON format from our API request. It has a field called `result`
// that will hold a list of `Article` objects.

#[derive(Clone, PartialEq, Deserialize)]
struct ArticleRequest {
    result: Vec<Article>,
}
// Represents an entire article, including its body, title, and a logo.
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Article {
    body: Vec<ArticleBody>,
    title: Option<String>,
    logo: ArticleLogo,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ArticleLogo {
    asset: ArticleLogoAsset,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ArticleLogoAsset {
    url: String,
}
// Represents one block of the article's body (e.g., text, images, etc.).
// Each block can have a style, children, or an asset if it's an image.
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ArticleBody {
    // `_type` in the JSON is renamed to `content_type` in Rust.
    #[serde(rename = "_type")]
    content_type: String,
    style: Option<String>,
    // `_key` in the JSON becomes `key`.
    #[serde(rename = "_key")]
    key: String,
    asset: Option<ArticleBodyAsset>,
    children: Option<Vec<ArticleBodyChild>>,
    level: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
// Represents a child element of the article body (e.g., a span of text).
struct ArticleBodyChild {
    #[serde(rename = "_type")]
    content_type: String,
    #[serde(rename = "_key")]
    key: String,
    text: String,
    marks: Vec<String>,
}

// Represents an asset (like an image) embedded in the body.
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ArticleBodyAsset {
    url: String,
    #[serde(rename = "mimeType")]
    mime_type: String,
}
// A helper function that takes an `Article` and converts its body into `Html` for rendering.
// Currently, it only looks for blocks of type "block" that contain spans, then renders them as paragraphs.
fn article_to_html(article: &Article) -> Html {
     // For debugging: log each block of the article body to the console.
    article.body.iter().for_each(|b| log!(format!("{:#?}", b)));

    html! {
        <>
            {
                for article.body.iter().map(|b| match b.content_type.as_str() {
                    "block" => html! {
                        {
                            for b.children.clone().unwrap_or(vec![]).iter().map(|c| match c.content_type.as_str() {
                                "span" => html! { <p class="mb-4 text-gray-700">{ c.text.clone() }</p> },
                                _ => html! {}
                            })
                        }
                    },
                    _ => html! {},
                })
            }
        </>
    }
}

// A function component called `ArticlePage` that displays a specific article.
// It retrieves data from an API based on the `slug` prop and displays the article contents.
#[function_component]
pub fn ArticlePage(props: &ArticlePageProps) -> Html {
     // Extract the `slug` from the component's props.
    let ArticlePageProps { slug } = props;

   // Get a navigator instance to allow us to go back or navigate to other pages.
    let navigator = use_navigator().expect("No navigator found!");

    // Definer 'go_back'-callback som kaller navigator.back().
     // Create a callback for our "Go Back" button. Calling `navigator.back()` will go to the previous page in history.
    let go_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.back();
        })
    };
// A piece of state (an Option<Article>) to store the fetched article. `None` means not loaded yet.
    let content = use_state(|| None);
    {
        let slug = slug.clone();
        let content = content.clone();
 // `use_effect_with` runs this side effect once on mount (since the dependency is `()`).
        // It fetches the article data based on the slug.
        use_effect_with((), move |_| {
            let slug = slug.clone();
            let content = content.clone();
 // Spawn an async task to perform the HTTP GET request.
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_content = Request::get(&format!("https://1fuw6fjt.api.sanity.io/v2022-03-07/data/query/production?query=++*%5B_type+%3D%3D+%22post%22+%26%26+slug.current+%3D%3D+%22{}%22%5D+%7B%0A++++++body%5B%5D+%7B%0A++++...%2C%0A++++asset-%3E%7B...%2C%22_key%22%3A+_id%7D%0A++%7D%2C%0A++title%2C%0A++logo+%7B%0A++++...%2C%0A++++asset-%3E%7B...%2C%22_key%22%3A+_id%7D%0A++%7D%0A++%7D+", slug))
                    .send()
                    .await
                    .unwrap()
                    .json::<ArticleRequest>()
                    .await
                    .unwrap();

                content.set(Some(
                    fetched_content
                        .result
                        .first()
                        .expect("Couldn't find an article")
                        .clone(),
                ));
            });
            // The cleanup function is empty here.
            || ()
        });
    }

    html! {
        <>
            {
                if let Some(content) = (*content).clone() {
                    html! {
                        <div class="container mx-auto max-w-4xl px-4 py-8 bg-white shadow-md rounded-lg">
                            <div class="flex flex-col sm:flex-row items-center gap-4 mb-6 border-b border-gray-200 pb-4">
                                <img
                                    class="w-16 h-16 sm:w-24 sm:h-24 rounded shadow-md"
                                    src={content.logo.asset.url.clone()}
                                    alt="Logo"
                                    loading="lazy"
                                />
                                <h1 class="text-3xl md:text-4xl lg:text-5xl text-gray-800 font-bold">
                                    { content.title.clone().unwrap_or_else(|| "Uten tittel".to_string()) }
                                </h1>
                            </div>

                            // Her bruker vi go_back-knappen
                            <button
                                onclick={go_back}
                                class="
                                    inline-block
                                    mb-4
                                    px-4
                                    py-2
                                    bg-gray-200
                                    text-gray-700
                                    rounded-md
                                    shadow
                                    hover:bg-gray-300
                                    transition-colors
                                "
                            >
                                { "← Go Back" }
                            </button>

                            <div class="prose prose-lg text-gray-700 leading-relaxed max-w-none">
                                { article_to_html(&content) }
                            </div>
                        </div>
                    }
                } else {
                    html! { <p class="text-center text-gray-600">{"Loading..."}</p> }
                }
            }
        </>
    }
}
