use chrono::NaiveDate;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::user_manager::{UserManager, UserState};

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
pub fn App() -> Html {
    let user_state = use_state(|| {
        UserManager::new(
            "TestUser".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
            "Test Person".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).expect("Couldn't parse naive date"),
        )
    });

    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let error_message = use_state(|| "".to_string());
    let blog_posts = use_state(|| Vec::<BlogPost>::new()); // State for blog posts

    // Automatically fetch blog posts on component load
    let blog_posts_clone = blog_posts.clone();
    spawn_local(async move {
        let res = reqwest::get("https://skby54ey.api.sanity.io/v2022-03-07/data/query/production?query=*[_type == \"post\"][0...3]{slug,title,\"logoUrl\":logo.asset->url}")
            .await;

        match res {
            Ok(response) => {
                let text = response.text().await.expect("Badly formatted API response");
                match serde_json::from_str::<ApiResponse>(&text) {
                    Ok(parsed_response) => {
                        blog_posts_clone.set(parsed_response.result); // Update blog posts
                    }
                    Err(err) => {
                        console::log_1(&format!("Parsing error: {}", err).into());
                    }
                }
            }
            Err(err) => {
                console::log_1(&format!("Fetch error: {}", err).into());
            }
        }
    });

    let on_login = {
        let user_state = user_state.clone();
        let email = email.clone();
        let password = password.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            let email = email.clone();
            let password = password.clone();
            let user_state = user_state.clone();
            let error_message = error_message.clone();

            spawn_local(async move {
                if let UserState::Unauthorized(manager) = &*user_state {
                    match manager.clone().login(&email, &password) {
                        Ok(new_state) => {
                            user_state.set(new_state);
                            error_message.set("".to_string());
                            console::log_1(&"User logged in successfully!".into());
                        }
                        Err(err) => {
                            error_message.set(err.clone());
                            console::log_1(&format!("Login error: {}", err).into());
                        }
                    }
                }
            });
        })
    };

    let on_logout = {
        let user_state = user_state.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            if let UserState::Authorized(manager) = &*user_state {
                user_state.set(manager.clone().logout());
                error_message.set("".to_string());
                console::log_1(&"User logged out.".into());
            }
        })
    };

    html! {
        <div style="font-family: Arial, sans-serif; padding: 20px; background-color: #f8f9fa;">
            {
                match &*user_state {
                    UserState::Authorized(user) => html! {
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
                                                <a href={format!("/post/{}", post.slug.current)}
                                                    style="
                                                    display: inline-block;
                                                    padding: 10px 16px;
                                                    background-color: #007bff;
                                                    color: white;
                                                    text-decoration: none;
                                                    border-radius: 4px;
                                                    font-size: 1rem;">{ "Read More" }</a>
                                            </li>
                                        })
                                    }
                                </ul>
                            </div>
                        </div>
                    },
                    UserState::Unauthorized(_) => html! {
                        <div style="display: flex; justify-content: center; align-items: center; height: 100vh; background-color: #f5f5f5;">
                            <div style="text-align: center; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);">
                                <h1 style="margin-bottom: 20px;">{ "Log in" }</h1>
                                <input
                                    type="email"
                                    placeholder="Email"
                                    value={(*email).clone()}
                                    oninput={Callback::from(move |e: InputEvent| email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                    style="padding: 8px; width: 100%; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px;"
                                />
                                <input
                                    type="password"
                                    placeholder="Password"
                                    value={(*password).clone()}
                                    oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                    style="padding: 8px; width: 100%; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px;"
                                />
                                <button onclick={on_login} style="padding: 10px 20px; font-size: 16px; background-color: #5cb85c; color: white; border: none; border-radius: 4px; cursor: pointer;">{ "Log in" }</button>
                                {
                                    if !(*error_message).is_empty() {
                                        html! { <p style="color: red; margin-top: 10px;">{ &*error_message }</p> }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        </div>
                    }
                }
            }
        </div>
    }
}
