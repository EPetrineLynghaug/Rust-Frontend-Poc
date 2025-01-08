// src/components.rs

use chrono::NaiveDate;
use std::env;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::user_manager::{UserManager, UserState};

#[function_component(App)]
pub fn app() -> Html {
    // Initialiserer brukerstate som Unauthorized
    let user_state = use_state(|| {
        UserManager::new(
            "TestUser".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
            "Test Person".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        )
    });

    // Tilstand for inputfelt
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let error_message = use_state(|| "".to_string());

    // Callback for innlogging
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

                            // Logg inn for debugging
                            console::log_1(&"User logged in successfully!".into());
                        }
                        Err(err) => {
                            error_message.set(err);
                            console::log_1(&format!("Login error: {}", err).into());
                        }
                    }
                }
            });
        })
    };

    // Callback for utlogging
    let on_logout = {
        let user_state = user_state.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            if let UserState::Authorized(manager) = &*user_state {
                user_state.set(manager.clone().logout());
                error_message.set("".to_string());

                // Logg ut for debugging
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
                            <h1>{ format!("Velkommen, {}!", user.get_name()) }</h1>
                            <button onclick={on_logout} style="margin-bottom: 20px;">{ "Logg ut" }</button>
                            <div>
                                <h2>{ "Du er innlogget." }</h2>
                                <div>
                                    {
                                        if !(*error_message).is_empty() {
                                            html! { <p style="color: red; margin-top: 10px;">{ &*error_message }</p> }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    },
                    UserState::Unauthorized(_) => html! {
                        <div>
                            <h1>{ "Logg inn" }</h1>
                            <div style="margin-bottom: 10px;">
                                <input
                                    type="email"
                                    placeholder="E-post"
                                    value={(*email).clone()}
                                    oninput={Callback::from(move |e: InputEvent| email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                    style="padding: 8px; width: 300px; margin-bottom: 10px;"
                                />
                            </div>
                            <div style="margin-bottom: 10px;">
                                <input
                                    type="password"
                                    placeholder="Passord"
                                    value={(*password).clone()}
                                    oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                    style="padding: 8px; width: 300px;"
                                />
                            </div>
                            <button onclick={on_login} style="padding: 10px 20px; font-size: 16px;">{ "Logg inn" }</button>
                            {
                                if !(*error_message).is_empty() {
                                    html! { <p style="color: red; margin-top: 10px;">{ &*error_message }</p> }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    }
                }
            }
        </div>
    }
}
