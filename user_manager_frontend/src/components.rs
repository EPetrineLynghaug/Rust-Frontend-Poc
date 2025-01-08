use crate::user_manager::{UserManager, UserState};
use chrono::NaiveDate;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
    let weather_data = use_state(|| None);

    let on_login = {
        let user_state = user_state.clone();
        let email = email.clone();
        let password = password.clone();
        let error_message = error_message.clone();
        let weather_data = weather_data.clone();

        Callback::from(move |_| {
            let email = email.clone();
            let password = password.clone();
            let user_state = user_state.clone();
            let error_message = error_message.clone();
            let weather_data = weather_data.clone();

            spawn_local(async move {
                if let UserState::Unauthorized(manager) = &*user_state {
                    match manager.clone().login(&email, &password) {
                        Ok(new_state) => {
                            user_state.set(new_state);
                            error_message.set("".to_string());

                            if let UserState::Authorized(_) = &*user_state {
                                match UserManager::fetch_weather(
                                    "Oslo",
                                    "fa94cfe79d4af2b4d631ec3ef0fd64ce",
                                )
                                .await
                                {
                                    Ok(data) => weather_data.set(Some(data)),
                                    Err(err) => error_message.set(err),
                                }
                            }
                        }
                        Err(err) => {
                            error_message.set(err);
                        }
                    }
                }
            });
        })
    };

    let on_logout = {
        let user_state = user_state.clone();
        let weather_data = weather_data.clone();

        Callback::from(move |_| {
            if let UserState::Authorized(manager) = &*user_state {
                user_state.set(manager.clone().logout());
                weather_data.set(None);
            }
        })
    };

    html! {
        <div>
            {
                match &*user_state {
                    UserState::Authorized(user) => html! {
                        <div>
                            <h1>{ format!("Velkommen, {}!", user.get_name()) }</h1>
                            <button onclick={on_logout}>{ "Logg ut" }</button>
                            <div>
                                <h2>{ "Værdata for Oslo:" }</h2>
                                {
                                    if let Some(weather) = &*weather_data {
                                        html! {
                                            <div>
                                                <p>{ format!("By: {}", weather.name) }</p>
                                                <p>{ format!("Temperatur: {}°C", weather.main.temp) }</p>
                                                <p>{ format!("Vær: {}", weather.weather[0].description) }</p>
                                            </div>
                                        }
                                    } else {
                                        html! { <p>{ "Henter værdata..." }</p> }
                                    }
                                }
                            </div>
                        </div>
                    },
                    UserState::Unauthorized(_) => html! {
                        <div>
                            <h1>{ "Logg inn" }</h1>
                            <input
                                type="email"
                                placeholder="E-post"
                                value={(*email).clone()}
                                oninput={Callback::from(move |e: InputEvent| email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                            />
                            <input
                                type="password"
                                placeholder="Passord"
                                value={(*password).clone()}
                                oninput={Callback::from(move |e: InputEvent| password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                            />
                            <button onclick={on_login}>{ "Logg inn" }</button>
                            {
                                if !error_message.is_empty() {
                                    html! { <p style="color: red;">{ &*error_message }</p> }
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
