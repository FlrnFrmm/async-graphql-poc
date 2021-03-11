#![feature(decl_macro, proc_macro_hygiene)]

mod models;
mod schemas;
mod database;

// use models::human;
// use schemas::humans::{HumanSchema, Query, Mutation};


use rocket::{response::content, State};

use juniper::{
    tests::fixtures::starwars::schema::{Database, Query},
    EmptyMutation, EmptySubscription, RootNode,
};

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket_async::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket_async::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket_async::GraphQLResponse {
    request.execute_sync(&schema, &context)
}


#[rocket::main]

async fn main() {
    // let hs = HumanSchema::new(Query, Mutation, juniper::EmptySubscription::new());

    // let context = schemas::humans::Context::new(database::DatabasePool{});

    // let (result, error) = juniper::execute_sync("query { apiVersion }", None, &hs, &juniper::Variables::new(), &context).unwrap();

    // println!("{:?}", result);

    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch()
        .await
        .expect("server to launch");
}

