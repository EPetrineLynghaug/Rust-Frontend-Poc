use gloo_console::log;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::Deserialize;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html};
use yew_router::prelude::*;

use crate::{app::Route, helpers::user_manager::UserState};

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct BlogPostsRequest {
    result: Vec<BlogPost>,
}

// Model for blog posts
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct BlogPost {
    slug: BlogSlug,
    title: String,
    #[serde(rename = "logoUrl")]
    logo_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct BlogSlug {
    current: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ApiResponse {
    result: Vec<BlogPost>, // Blog posts are stored in "result"
}

#[function_component]
pub fn HomePage() -> Html {
    let navigator = use_navigator().expect("Couldn't get the navigator");

    if LocalStorage::get::<bool>("login").unwrap_or_else(|_| false) == false {
        log!("Not logged in");
        navigator.replace(&Route::Login);
    }

    let local_storage_user_state = match LocalStorage::get::<UserState>("login_state") {
        Ok(user_state) => user_state,
        Err(_) => {
            LocalStorage::set::<bool>("login", false).expect("Couldn't save login toggle!");

            navigator.replace(&Route::Login);
            return html! { <p>{ "Internal error with user handeling!" }</p> };
        }
    };
    let user_state = use_state(move || local_storage_user_state);

    let user = match &*user_state {
        UserState::Authorized(user) => user.clone(),
        UserState::Unauthorized(_) => {
            navigator.replace(&Route::Login);
            return html! { <p>{ "Internal error with user handeling!" }</p> };
        }
    };

    let error_message = use_state(|| "".to_string());
    let blog_posts = use_state(|| Vec::<BlogPost>::new()); // State for blog posts

    // Automatically fetch blog posts on component load
    {
        let blog_posts = blog_posts.clone();

        use_effect_with((), move |_| {
            let blog_posts = blog_posts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts = Request::get("https://skby54ey.api.sanity.io/v2022-03-07/data/query/production?query=*[_type == \"post\"][0...3]{slug,title,\"logoUrl\":logo.asset->url}")
                    .send()
                    .await
                    .unwrap()
                    .json::<BlogPostsRequest>()
                    .await
                    .unwrap();

                blog_posts.set(fetched_posts.result);
            });
            || ()
        });
    }

    let on_logout = {
        let user_state = user_state.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            if let UserState::Authorized(manager) = &*user_state {
                user_state.set(manager.clone().logout());
                error_message.set("".to_string());
                log!("User logged out.");
            }
        })
    };

    html! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                <h1 style="font-size: 2.5rem; color: #333;">{ format!("Welcome, {}!", user.get_name()) }</h1>
                <button onclick={on_logout} style="padding: 10px 20px; background-color: #d9534f; color: white; border: none; border-radius: 4px; cursor: pointer;">{ "Log out" }</button>
            </div>
            <div>
                <h2 style="font-size: 1.8rem; color: #333; margin-bottom: 20px;">{ "The Rust Blog" }</h2>
                <ul style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; list-style: none; padding: 0; margin: 0;">
                    {
                        for blog_posts.iter().map(|post| html! {
                            <li style="
                                background: white;
                                padding: 20px;
                                border-radius: 8px;
                                box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
                                transition: transform 0.3s ease, box-shadow 0.3s ease;
                                cursor: pointer;">
                                {
                                    if let Some(url) = &post.logo_url {
                                        html! { <img src={url.clone()} alt="Blog image" style="width: 100%; height: auto; border-radius: 8px; margin-bottom: 16px;" /> }
                                    } else {
                                        html! {}
                                    }
                                }
                                <h3 style="font-size: 1.5rem; color: #333; margin-bottom: 10px;">{ &post.title }</h3>
                                <Link<Route> to={Route::Article { slug: post.slug.current.clone() }}>{ "Read More" }</Link<Route>>
                            </li>
                        })
                    }
                </ul>
            </div>
        </div>
    }
}
