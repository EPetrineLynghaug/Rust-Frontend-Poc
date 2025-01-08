// src/user_manager.rs

use chrono::NaiveDate;
use implicit_clone::ImplicitClone;
use sha2::{Digest, Sha256};
use std::marker::PhantomData;
use web_sys::console; // For logging

/// Enum representing the state of a user.
#[derive(Clone)]
pub enum UserState {
    Unauthorized(UserManager<Unauthorized>),
    Authorized(UserManager<Authorized>),
}

/// Struct representing an authorized user.
#[derive(Clone)]
pub struct Authorized;

/// Struct representing an unauthorized user.
#[derive(Clone)]
pub struct Unauthorized;

/// Generic struct for managing user data, parameterized by state.
#[derive(Clone)]
pub struct UserManager<State = Unauthorized> {
    state: PhantomData<State>,
    username: String,
    email: String,
    password_hash: String,
    name: String,
    birthday: NaiveDate,
}

impl UserManager<Authorized> {
    /// Get the user's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Logout the user, transitioning to Unauthorized state.
    pub fn logout(self) -> UserState {
        UserState::Unauthorized(UserManager {
            state: PhantomData,
            username: self.username,
            email: self.email,
            password_hash: self.password_hash,
            name: self.name,
            birthday: self.birthday,
        })
    }
}

impl UserManager<Unauthorized> {
    /// Login the user with email and password.
    pub fn login(self, email: &str, password: &str) -> Result<UserState, String> {
        if email != self.email || UserManager::hash_password(password) != self.password_hash {
            console::log_1(&"Login failed: Invalid email or password.".into());
            Err("Feil e-post eller passord".to_string())
        } else {
            console::log_1(&"Login succeeded.".into());
            Ok(UserState::Authorized(UserManager {
                state: PhantomData,
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
    /// Create a new UserManager instance, initially in Unauthorized state.
    pub fn new(
        username: String,
        email: String,
        password: String,
        name: String,
        birthday: NaiveDate,
    ) -> UserState {
        let password_hash = Self::hash_password(&password);
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

    /// Hash a password using SHA256.
    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        format!("{:x}", hasher.finalize())
    }
}
