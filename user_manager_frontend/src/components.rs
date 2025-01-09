// src/components.rs

use chrono::NaiveDate;

use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::user_manager::{UserManager, UserState};

#[function_component(App)]
pub fn app() -> Html {
    let user_state = use_state(|| {
        UserManager::new(
            "TestUser".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
            "Test Person".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        )
    });

    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let error_message = use_state(|| "".to_string());

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

    let sanity = {
        Callback::from(move |_| {
            // console::log_1(&"Sanity check".into());

            wasm_bindgen_futures::spawn_local(async move {
                let res = reqwest::get("https://skby54ey.api.sanity.io/v2022-03-07/data/query/production?query=*%5B_type+%3D%3D+%22post%22%5D%5B0...3%5D%7B%0A++slug%2C%0A++title%2C%0A++%22logoUrl%22%3A+logo.asset-%3Eurl%0A%7D")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                // assert_eq!(res.status(), 200);

                console::log_1(&res.into());
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
        <div style="font-family: Arial, sans-serif; padding: 20px;">
            {
                match &*user_state {
                    UserState::Authorized(user) => html! {
                        <div>
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                                <h1>{ format!("Velkommen, {}!", user.get_name()) }</h1>
                                <button onclick={on_logout} style="padding: 8px 16px; background-color: #d9534f; color: white; border: none; border-radius: 4px; cursor: pointer;">{ "Logg ut" }</button>
                                <button onclick={sanity} style="padding: 8px 16px; background-color: #5cb85c; color: white; border: none; border-radius: 4px; cursor: pointer;">{ "Sanity" }</button>
                            </div>
                            <div style="background-color: #f9f9f9; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);">
                                <h2>{ "Din Dashboard" }</h2>
                                <p>{ "Her kan du bygge ut applikasjonen videre med flere funksjoner." }</p>
                            </div>
                        </div>
                    },
                    UserState::Unauthorized(_) => html! {
                        <div style="display: flex; justify-content: center; align-items: center; height: 100vh; background-color: #f5f5f5;">
                            <div style="text-align: center; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);">
                                <h1 style="margin-bottom: 20px;">{ "Logg inn" }</h1>
                                <input
                                    type="email"
                                    placeholder="E-post"
                                    value={(*email).clone()}
                                    oninput={Callback::from(move |e: InputEvent| email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                    style="padding: 8px; width: 100%; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px;"
                                />
                                <input
                                    type="password"
                                    placeholder="Passord"
                                    value={(*password).clone()}
                                    oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                    style="padding: 8px; width: 100%; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px;"
                                />
                                <button onclick={on_login} style="padding: 10px 20px; font-size: 16px; background-color: #5cb85c; color: white; border: none; border-radius: 4px; cursor: pointer;">{ "Logg inn" }</button>
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
