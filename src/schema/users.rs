use async_graphql::Object;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user_query_test(&self) -> String {
        "User Query Test".into()
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