use chrono::NaiveDate;
use sha2::{Digest, Sha256};
use std::marker::PhantomData;
use web_sys::console;

/// Representerer brukertilstanden: Ulogget eller Innlogget.
#[derive(Clone)]
pub enum UserState {
    Unauthorized(UserManager<Unauthorized>),
    Authorized(UserManager<Authorized>),
}

/// Skilletyper for unauthorized/authorized.
#[derive(Clone)]
pub struct Unauthorized;

#[derive(Clone)]
pub struct Authorized;

/// Struktur for brukerhåndtering.
#[derive(Clone)]
pub struct UserManager<State = Unauthorized> {
    state: PhantomData<State>,
    username: String,
    email: String,
    password_hash: String,
    name: String,
    birthday: NaiveDate,
}

impl UserManager<Unauthorized> {
    /// `login(&self, ...)` – tar en referanse.
    /// Lager en `Authorized`-tilstand ved å klone feltene hvis passordet stemmer.
    pub fn login(&self, email: &str, password: &str) -> Result<UserState, String> {
        if email != self.email || Self::hash_password(password) != self.password_hash {
            console::log_1(&"Login failed: Invalid email or password.".into());
            Err("Feil e-post eller passord".to_string())
        } else {
            console::log_1(&"Login succeeded.".into());
            Ok(UserState::Authorized(UserManager {
                state: PhantomData,
                username: self.username.clone(),
                email: self.email.clone(),
                password_hash: self.password_hash.clone(),
                name: self.name.clone(),
                birthday: self.birthday,
            }))
        }
    }
}

impl UserManager<Authorized> {
    /// Hent brukernavnet
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// `logout(&self)` – tar en referanse.
    /// Lager en `Unauthorized` ved å klone datafeltene.
    pub fn logout(&self) -> UserState {
        console::log_1(&"Logout succeeded.".into());
        UserState::Unauthorized(UserManager {
            state: PhantomData,
            username: self.username.clone(),
            email: self.email.clone(),
            password_hash: self.password_hash.clone(),
            name: self.name.clone(),
            birthday: self.birthday,
        })
    }
}

impl UserManager {
    /// Oppretter en ny bruker i Unauthorized state.
    pub fn new(
        username: impl Into<String>,
        email: impl Into<String>,
        password: impl AsRef<str>,
        name: impl Into<String>,
        birthday: NaiveDate,
    ) -> UserState {
        let username = username.into();
        let email = email.into();
        let name = name.into();
        let password_hash = Self::hash_password(password.as_ref());
        console::log_1(&"Created new UserManager in Unauthorized state.".into());

        UserState::Unauthorized(UserManager {
            state: PhantomData,
            username,
            email,
            password_hash,
            name,
            birthday,
        })
    }

    /// Enkel SHA256-hashing.
    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        format!("{:x}", hasher.finalize())
    }
}
