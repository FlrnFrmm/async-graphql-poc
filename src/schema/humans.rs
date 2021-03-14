use async_graphql::Object;

#[derive(Default)]
pub struct HumanQuery;

#[Object]
impl HumanQuery {
    async fn human_query_test(&self) -> String {
        "Human Query Test".into()
    }
}

#[derive(Default)]
pub struct HumanMutation;

#[Object]
impl HumanMutation {
    async fn human_mutation_test(&self) -> String {
        "Human Mutation Test".into()
    }
}