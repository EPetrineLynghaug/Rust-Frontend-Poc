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
                                "span" => html! { <p>{ c.text.clone() }</p> },
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
                if content.is_some() {
                    let content = <std::option::Option<Article> as Clone>::clone(&content).unwrap();

                    html! {
                        <>
                            <h1>{ content.title.clone() }</h1>

                            <img src={content.logo.asset.url.clone()} />

                            { article_to_html(&content) }
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
