use gloo_console::log;
// This import allows you to make HTTP requests (e.g., GET, POST) from Rust in a web environment.
use gloo_net::http::Request;
// Here we import features to interact with the browser's local storage (LocalStorage).
use gloo_storage::{LocalStorage, Storage};
// `serde` is a framework for serializing and deserializing data. `Deserialize` helps decode JSON into Rust types.
use serde::Deserialize;

// Below are parts of the Yew framework:
// - `function_component` for creating a functional component
// - `html` for writing HTML in Rust using a JSX-like syntax
// - `use_effect_with` for running side effects
// - `use_state` for state management
// - `Callback` and `Html` are utility types
use yew::{function_component, html, use_effect_with, use_state, Callback, Html};
// This import gives us the ability to navigate between pages/routes in a Yew application.
use yew_router::prelude::*;

use crate::{app::Route, helpers::user_manager::UserState};


// We define a struct that matches the shape of the JSON response for fetching blog posts (outer container).
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct BlogPostsRequest {
    result: Vec<BlogPost>,
}

// This struct represents individual blog posts.
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

// A Yew function component called `HomePage`. It shows a homepage view.
#[function_component]
pub fn HomePage() -> Html {
    // Navigator lets us programmatically go to different routes.
    let navigator = use_navigator().expect("Couldn't get the navigator");

    // Check if the user is logged in by reading a boolean from local storage.
    // If the value isn't `true`, we log a message and redirect to the login page.
    if LocalStorage::get::<bool>("login").unwrap_or_else(|_| false) == false {
        log!("Not logged in");
        navigator.replace(&Route::Login);
    }
// Try to get the user's state (e.g., Authorized, Unauthorized) from local storage.
    // If we fail, we set "login" to false and redirect to the login page.
    let local_storage_user_state = match LocalStorage::get::<UserState>("login_state") {
        Ok(user_state) => user_state,
        Err(_) => {
            LocalStorage::set::<bool>("login", false).expect("Couldn't save login toggle!");

            navigator.replace(&Route::Login);
            return html! { <p>{ "Internal error with user handeling!" }</p> };
        }
    };
     // `user_state` holds the current user's state. We initialize it from what we got above.
    let user_state = use_state(move || local_storage_user_state);
// We check if the user is actually authorized. If not, we send them to Login.
    let user = match &*user_state {
        UserState::Authorized(user) => user.clone(),
        UserState::Unauthorized(_) => {
            navigator.replace(&Route::Login);
            return html! { <p>{ "Internal error with user handeling!" }</p> };
        }
    };
  // A place to store any error messages that might occur.
    let error_message = use_state(|| "".to_string());
    // This will store the list of blog posts once we fetch them.
    let blog_posts = use_state(|| Vec::<BlogPost>::new());

// This block uses a Yew "effect" hook to perform an asynchronous fetch of blog posts.
    {
        let blog_posts = blog_posts.clone();
 //`use_effect_with` runs a side effect when its dependencies change. Here, the dependencies are `()`, which never changes,
        // so it runs once when the component first loads.
        use_effect_with((), move |_| {
            let blog_posts = blog_posts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                   // Make an HTTP GET request to fetch the first 3 blog posts from our API.
                let fetched_posts = Request::get("https://1fuw6fjt.api.sanity.io/v2022-03-07/data/query/production?query=*[_type == \"post\"][0...3]{slug,title,\"logoUrl\":logo.asset->url}")
                    .send()
                    .await
                    .unwrap()
                     // Convert the JSON response into our `BlogPostsRequest` struct.
                    .json::<BlogPostsRequest>()
                    .await
                    .unwrap();
 // Update the `blog_posts` state with the fetched blog posts.
                blog_posts.set(fetched_posts.result);
            });
            || ()
        });
    }
// Callback that runs when the user clicks the "Log out" button.
    let on_logout = {
        let user_state = user_state.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            // We only log out if the user is currently authorized.
            if let UserState::Authorized(manager) = &*user_state {
                // `logout()` will return a new `UserState` (Unauthorized).
                user_state.set(manager.clone().logout());
                error_message.set("".to_string());
                log!("User logged out.");
// Redirect the user to the login page after logging out.
                navigator.replace(&Route::Login);
            }
        })
    };

    html! {
        <div class="max-w-[1200px] mx-auto p-4 box-border">
       <section class="relative bg-cover bg-center h-64 rounded-lg mb-8" style="background-image: url('https://via.placeholder.com/1200x400');">
        <div class="absolute inset-0 bg-gray-900 bg-opacity-50 rounded-lg"></div>
        <div class="relative flex items-center justify-center h-full">
            <div class="text-center">
                <h2 class="text-4xl text-white font-bold mb-2">{ "Welcome to the Rust Blog" }</h2>
                <p class="text-lg text-gray-300">{ "Explore the latest posts on Rust programming and more!" }</p>
            </div>
        </div>
    </section>

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
                    { "Latest Posts" }
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
