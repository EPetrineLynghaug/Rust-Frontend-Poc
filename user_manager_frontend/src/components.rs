use chrono::NaiveDate;
use gloo_net::http::Request;
use implicit_clone::unsync::IMap;
use indexmap::map::IndexMap;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::user_manager::{UserManager, UserState};

#[derive(Clone, PartialEq, Deserialize, Debug, implicit_clone::ImplicitClone)]
pub struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

pub type PostMap = IMap<u32, Post>;

async fn fetch_posts() -> Result<IndexMap<u32, Post>, String> {
    let resp = Request::get("https://jsonplaceholder.typicode.com/posts?_limit=5")
        .send()
        .await
        .map_err(|err| format!("Nettverksfeil: {err}"))?;

    let posts_vec: Vec<Post> = resp
        .json()
        .await
        .map_err(|err| format!("JSON parse error: {err}"))?;

    let mut map = IndexMap::new();
    for post in posts_vec {
        map.insert(post.id, post);
    }
    Ok(map)
}

#[function_component(App)]
pub fn app() -> Html {
    // 1) Brukertilstand
    let user_state = use_state(|| {
        // Oppretter en "JohnDoe" i Unauthorized-tilstand
        let bday = NaiveDate::from_ymd_opt(1990, 1, 1).expect("Invalid date");
        UserManager::new("JohnDoe", "john@doe.com", "password123", "John Doe", bday)
    });

    // 2) Oppbevar poster i en IMap
    let posts = use_state(PostMap::default);

    // 3) on_login
    let on_login = {
        let user_state = user_state.clone();
        Callback::from(move |_| {
            if let UserState::Unauthorized(um) = &*user_state {
                match um.login("john@doe.com", "password123") {
                    Ok(new_state) => user_state.set(new_state),
                    Err(err) => console::log_1(&format!("Login error: {err}").into()),
                }
            }
        })
    };

    // 4) on_logout
    let on_logout = {
        let user_state = user_state.clone();
        Callback::from(move |_| {
            if let UserState::Authorized(um) = &*user_state {
                user_state.set(um.logout());
            }
        })
    };

    // 5) on_fetch_posts
    let on_fetch_posts = {
        let posts = posts.clone();
        Callback::from(move |_| {
            spawn_local(async move {
                match fetch_posts().await {
                    Ok(index_map) => {
                        posts.set(PostMap::from(index_map));
                    }
                    Err(e) => console::log_1(&format!("fetch_posts error: {e}").into()),
                }
            });
        })
    };

    // 6) HTML-rendering
    html! {
        <main>
            <h1>{ "Rust Yew Eksempel – Innlogging & API" }</h1>
            {
                match &*user_state {
                    UserState::Unauthorized(_) => html! {
                        <>
                            <p>{ "Du er ikke logget inn." }</p>
                            <button onclick={on_login}>{ "Logg inn" }</button>
                        </>
                    },
                    UserState::Authorized(user) => html! {
                        <>
                            <p>{ format!("Hei, {}!", user.get_name()) }</p>
                            <button onclick={on_logout}>{ "Logg ut" }</button>
                            <button onclick={on_fetch_posts}>{ "Hent poster" }</button>
                        </>
                    },
                }
            }
            {
                if posts.is_empty() {
                    html! { <p>{ "Ingen poster. Klikk \"Hent poster\"." }</p> }
                } else {
                    html! {
                        <ul>
                            { for posts.iter().map(|(id, post)| html! {
                                <li key={(*id).to_string()}>
                                    <strong>{ format!("Post #{}: {}", post.id, post.title) }</strong>
                                    <p>{ &post.body }</p>
                                </li>
                            }) }
                        </ul>
                    }
                }
            }
        </main>
    }
}
