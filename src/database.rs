mod human;
mod user;

use crate::models::user::User;

pub struct DatabasePool {
    users: Vec<User>
}

impl DatabasePool {
    pub fn new() -> DatabasePool {
        Self { users: Vec::new() }
    }
}

impl DatabasePool {
}