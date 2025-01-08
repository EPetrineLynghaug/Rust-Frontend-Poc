use chrono::NaiveDate;
use gloo_net::http::Request;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub enum UserState {
    Unauthorized(UserManager<Unauthorized>),
    Authorized(UserManager<Authorized>),
}

#[derive(Clone)]
pub struct Authorized;
#[derive(Clone)]
pub struct Unauthorized;

#[derive(Clone)]
pub struct UserManager<State = Unauthorized> {
    state: std::marker::PhantomData<State>,
    username: String,
    email: String,
    password_hash: String,
    name: String,
    birthday: NaiveDate,
}

impl UserManager<Authorized> {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn logout(self) -> UserState {
        UserState::Unauthorized(UserManager {
            state: std::marker::PhantomData,
            username: self.username,
            email: self.email,
            password_hash: self.password_hash,
            name: self.name,
            birthday: self.birthday,
        })
    }

    pub async fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse, String> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, api_key
        );

        let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            response
                .json::<WeatherResponse>()
                .await
                .map_err(|e| e.to_string())
        } else {
            Err(format!("API-feil: {}", response.status()))
        }
    }
}

impl UserManager<Unauthorized> {
    pub fn login(self, email: &str, password: &str) -> Result<UserState, String> {
        if email != self.email || UserManager::hash_password(password) != self.password_hash {
            Err("Feil e-post eller passord".to_string())
        } else {
            Ok(UserState::Authorized(UserManager {
                state: std::marker::PhantomData,
                username: self.username,
                email: self.email,
                password_hash: self.password_hash,
                name: self.name,
                birthday: self.birthday,
            }))
        }
    }
}

impl UserManager {
    pub fn new(
        username: String,
        email: String,
        password: String,
        name: String,
        birthday: NaiveDate,
    ) -> UserState {
        let password_hash = Self::hash_password(&password);
        UserState::Unauthorized(UserManager {
            state: Default::default(),
            username,
            email,
            password_hash,
            name,
            birthday,
        })
    }

    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}
