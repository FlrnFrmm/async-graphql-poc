use async_graphql::{SimpleObject, Enum};

#[derive(SimpleObject)]
pub struct User {
    pub avatar_url: String,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub token: String,
    pub wallet_address: String,  
    pub settings: Settings
}

#[derive(SimpleObject)]
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