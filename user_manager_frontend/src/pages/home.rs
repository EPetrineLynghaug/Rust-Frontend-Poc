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
    let blog_posts = use_state(|| Vec::<BlogPost>::new());

    {
        let blog_posts = blog_posts.clone();

        use_effect_with((), move |_| {
            let blog_posts = blog_posts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts = Request::get("https://1fuw6fjt.api.sanity.io/v2022-03-07/data/query/production?query=*[_type == \"post\"][0...3]{slug,title,\"logoUrl\":logo.asset->url}")
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
        let navigator = navigator.clone();

        Callback::from(move |_| {
            if let UserState::Authorized(manager) = &*user_state {
                user_state.set(manager.clone().logout());
                error_message.set("".to_string());
                log!("User logged out.");

                navigator.replace(&Route::Login);
            }
        })
    };

    html! {
        <div class="max-w-[1200px] mx-auto p-4 box-border">

            <div class="flex flex-wrap justify-between items-center mb-5">
                <h1 class="text-[clamp(1.5rem,5vw,2.5rem)] text-gray-900 font-bold m-0">
                    { format!("Welcome, {}!", user.get_name()) }
                </h1>
                <button
                    onclick={on_logout}
                    class="px-6 py-3 bg-[#d9534f] text-white font-semibold rounded-lg cursor-pointer
                           transition-all duration-300
                           hover:bg-[#c9302c] hover:shadow-md"
                >
                    { "Log out" }
                </button>
            </div>


            <div>
                <h2 class="text-[clamp(1.2rem,4vw,1.8rem)] text-gray-800 font-semibold mb-5 mt-0">
                    { "The Rust Blog" }
                </h2>


                <ul class="
                    grid grid-cols-[repeat(auto-fill,_minmax(300px,_1fr))]
                    gap-6
                    list-none
                    p-0
                    m-0
                ">
                    {
                        for blog_posts.iter().map(|post| {
                            html! {
                                <li
                                    class="
                                        bg-white
                                        p-6
                                        rounded-xl
                                        shadow-lg
                                        transition-transform
                                        duration-300
                                        ease-in-out
                                        cursor-pointer
                                        hover:shadow-xl
                                        hover:scale-[1.03]
                                    "
                                >
                                    {
                                        if let Some(url) = &post.logo_url {
                                            html! {
                                                <img
                                                    src={url.clone()}
                                                    alt="Blog image"
                                                    class="w-full h-auto rounded-xl mb-4"
                                                />
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    <h3 class="text-lg text-gray-900 font-bold mb-3 mt-0">
                                        { &post.title }
                                    </h3>
                                    <Link<Route>
                                    to={Route::Article { slug: post.slug.current.clone() }}
                                    classes="
                                        inline-block
                                        px-4
                                        py-2
                                        bg-blue-500
                                        text-white
                                        text-sm
                                        font-medium
                                        rounded-md
                                        shadow-sm
                                        hover:bg-blue-600
                                        transition-colors
                                    "
                                >
                                    { "Read More" }
                                </Link<Route>>


                                </li>
                            }
                        })
                    }
                </ul>
            </div>
        </div>
    }
}
