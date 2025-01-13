use chrono::NaiveDate;
use gloo_console::{error, log};
use gloo_storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, TargetCast};
use yew_router::hooks::use_navigator;

use crate::{
    app::Route,
    helpers::user_manager::{UserManager, UserState},
};

#[function_component]
pub fn LoginPage() -> Html {
    let navigator = use_navigator().expect("Couldn't get the navigator");

    if LocalStorage::get::<bool>("login").unwrap_or_else(|_| false) == true {
        log!("Already logged in");
        navigator.replace(&Route::Home);
    }

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

    let on_login = {
        let user_state = user_state.clone();
        let email = email.clone();
        let password = password.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            if let UserState::Unauthorized(manager) = &*user_state {
                match manager.clone().login(&email, &password) {
                    Ok(new_state) => {
                        user_state.set(new_state);
                        error_message.set("".to_string());
                        log!("User logged in successfully!");

                        navigator.push(&Route::Home);
                    }
                    Err(err) => {
                        error_message.set(err.clone());
                        error!("Login error", err);
                    }
                }
            }
        })
    };

    html! {
        <div style="font-family: Arial, sans-serif; padding: 20px; background-color: #f8f9fa;">
            <div style="display: flex; justify-content: center; align-items: center; height: 100vh; background-color: #f5f5f5;">
                <div style="text-align: center; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);">
                    <h1 style="margin-bottom: 20px;">{ "Log in" }</h1>
                    <input
                        type="email"
                        placeholder="Email"
                        value={(*email).clone()}
                        oninput={Callback::from(move |e: InputEvent| email.set(e.target_unchecked_into::<HtmlInputElement>().value()))}
                        style="padding: 8px; width: 100%; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px;"
                    />
                    <input
                        type="password"
                        placeholder="Password"
                        value={(*password).clone()}
                        oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<HtmlInputElement>().value()))}
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
        </div>
    }
}
