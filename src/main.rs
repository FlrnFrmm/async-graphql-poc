mod models;
mod schemas;
mod database;

use models::human;
use schemas::humans::{HumanSchema, Query, Mutation};

fn main() {
    let hs = HumanSchema::new(Query, Mutation, juniper::EmptySubscription::new());

    let context = schemas::humans::Context::new(database::DatabasePool{});

    let (result, error) = juniper::execute_sync("query { apiVersion }", None, &hs, &juniper::Variables::new(), &context).unwrap();

    println!("{:?}", result);
}
