use chrono::NaiveDate;

use gloo_console::log; // For logging
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::marker::PhantomData;

/// Enum representing the state of a user.
#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Clone, Serialize, Deserialize)]
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
        LocalStorage::set::<bool>("login", false).expect("Couldn't save login toggle!");
        LocalStorage::delete("login_state");

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
            log!("Login failed: Invalid email or password.");
            Err("Feil e-post eller passord".to_string())
        } else {
            log!("Login succeeded.");

            let new_state = UserState::Authorized(UserManager {
                state: PhantomData,
                username: self.username,
                email: self.email,
                password_hash: self.password_hash,
                name: self.name,
                birthday: self.birthday,
            });

            LocalStorage::set::<bool>("login", true).expect("Couldn't save login toggle!");
            LocalStorage::set::<UserState>("login_state", new_state.clone())
                .expect("Couldn't save login state!");

            Ok(new_state)
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
        log!("Created new UserManager in Unauthorized state.");
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
