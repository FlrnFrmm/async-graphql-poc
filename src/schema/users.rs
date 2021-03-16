use async_graphql::{Object, Context, Result};

use crate::{database::DatabaseContext, models::user::User};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self, context: &Context<'_>, id: i32) -> Result<Option<User>> {
        let dbc = context.data::<DatabaseContext>()?;
        Ok(dbc.get_user(id).await?)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn user_mutation_test(&self) -> String {
        "User Mutation Test".into()
    }
}

pub struct SignUpInput {
    name: String,
    email: String,
    password: String,
}