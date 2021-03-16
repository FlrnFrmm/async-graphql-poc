use async_graphql::{SimpleObject, Enum};
use sqlx::FromRow;

#[derive(SimpleObject, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    // pub settings: Settings,
    // pub avatar_url: Option<String>,
    // pub bio: Option<String>,
    // pub wallet_address: Option<String>
}

impl User {
     pub fn new(name: &str, password: &str, email: &str) -> Self {
        Self { 
            id: 0, 
            name: name.into(), 
            password: password.into(), 
            email: email.into(), 
        } //settings, avatar_url, bio, wallet_address }
     }
 }

#[derive(SimpleObject, Default)]
pub struct Settings {
    pub mode: Mode,
    pub remember_me: bool,
    pub newsletter: bool
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Light, 
    Dark
}

impl Default for Mode {
    fn default() -> Self {
        Self::Light
    }
}