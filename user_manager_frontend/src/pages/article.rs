use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::{function_component, html, use_effect_with, use_state, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ArticlePageProps {
    pub slug: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct ArticleRequest {
    result: Vec<Article>,
}

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

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ArticleBody {
    #[serde(rename = "_type")]
    content_type: String,
    style: Option<String>,
    #[serde(rename = "_key")]
    key: String,
    asset: Option<ArticleBodyAsset>,
    children: Option<Vec<ArticleBodyChild>>,
    level: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ArticleBodyChild {
    #[serde(rename = "_type")]
    content_type: String,
    #[serde(rename = "_key")]
    key: String,
    text: String,
    marks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ArticleBodyAsset {
    url: String,
    #[serde(rename = "mimeType")]
    mime_type: String,
}

fn article_to_html(article: &Article) -> Html {
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

#[function_component]
pub fn ArticlePage(props: &ArticlePageProps) -> Html {
    let ArticlePageProps { slug } = props;

    let content = use_state(|| None);
    {
        let slug = slug.clone();
        let content = content.clone();

        use_effect_with((), move |_| {
            let slug = slug.clone();
            let content = content.clone();

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
