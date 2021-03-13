pub mod humans;

use juniper::EmptySubscription;

use crate::database::DatabasePool;

pub struct Query;

pub struct Mutation;

pub struct Context {
    // Use your real database pool here.
    pool: DatabasePool,
}

impl Context {
    pub fn new() -> Self {
        Self { pool: DatabasePool{} }
    }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context { }

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn new() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}