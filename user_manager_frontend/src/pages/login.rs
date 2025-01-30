// This line imports the `NaiveDate` type from the `chrono` library.
// `NaiveDate` is used to work with dates without any time or timezone information.
use chrono::NaiveDate;
// These imports provide logging (`log` and `error`) and local storage functionality (`LocalStorage`).
use gloo_console::{error, log};
use gloo_storage::{LocalStorage, Storage};

// This import allows us to interact with the <input> elements in the browser.
use web_sys::HtmlInputElement;

// These imports are core parts of the Yew framework. They let us create components, manage state, and handle events.
use yew::{function_component, html, use_state, Callback, Html, InputEvent, TargetCast};

// This import allows us to navigate between different pages or routes in a Yew application.
use yew_router::hooks::use_navigator;

// Here we import the `Route` enum (or struct) that defines different pages in our app,
// and also the `UserManager` and `UserState` which handle user-related logic such as logging in.
use crate::{
    app::Route,
    helpers::user_manager::{UserManager, UserState},
};

#[function_component]
pub fn LoginPage() -> Html {
    // `use_navigator` gives us a way to navigate between routes (pages) in our web application.
    let navigator = use_navigator().expect("Couldn't get the navigator");

    // This checks if the user is already logged in by reading a boolean from the browser's Local Storage.
    // If it's `true`, we log a message and redirect the user to the Home page without showing the login form.
    if LocalStorage::get::<bool>("login").unwrap_or_else(|_| false) == true {
        log!("Already logged in");
        navigator.replace(&Route::Home);
    }

    // Create a piece of state (`user_state`) that holds an initial "unauthorized" user
    // with some default test data (username, email, password, name, and birth date).
    let user_state = use_state(|| {
        UserManager::new(
            "TestUser".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
            "Test Person".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).expect("Couldn't parse naive date"),
        )
    });

    // Two pieces of state to store whatever the user types in the email and password fields.
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let error_message = use_state(|| "".to_string());

    // This callback is triggered when the user clicks the "Log in" button.
    let on_login = {
        let user_state = user_state.clone();
        let email = email.clone();
        let password = password.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            // We only attempt to log in if the current user state is `Unauthorized`.
            if let UserState::Unauthorized(manager) = &*user_state {
                // Call the `login` method on our `UserManager`, passing in the email and password.
                match manager.clone().login(&email, &password) {
                    // If login is successful, we set a new user state and clear the error message.
                    Ok(new_state) => {
                        user_state.set(new_state);
                        error_message.set("".to_string());
                        log!("User logged in successfully!");
                        // After successful login, navigate to the Home page.
                        navigator.push(&Route::Home);
                    }
                     // If there's an error, we display it and log it to the console.
                    Err(err) => {
                        error_message.set(err.clone());
                        error!("Login error", err);
                    }
                }
            }
        })
    };
// The HTML (using Yew's JSX-like syntax) that we render for the login page.
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
