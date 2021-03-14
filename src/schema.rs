pub mod humans;
pub mod users;

use std::default::Default;
use async_graphql::{MergedObject, EmptySubscription};

#[derive(MergedObject, Default)]
pub struct Query(humans::HumanQuery, users::UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(humans::HumanMutation, users::UserMutation);

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub fn new() -> Schema {
    async_graphql::Schema::new(Query::default(), Mutation::default(), EmptySubscription)
}