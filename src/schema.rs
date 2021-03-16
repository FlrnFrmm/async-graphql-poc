pub mod users;

use std::default::Default;
use async_graphql::{MergedObject, EmptySubscription};
use anyhow::Result;

use crate::database::DatabaseContext;

#[derive(MergedObject, Default)]
pub struct Query(users::UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(users::UserMutation);

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub async fn new() -> Result<Schema> {
    let schema = 
        async_graphql::Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(DatabaseContext::new().await?)
            .finish();

    Ok(schema)
}