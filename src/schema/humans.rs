#![allow(unused_variables)]

use crate::models::human::{Episode, Human};
use std::fmt::Display;

use juniper::{graphql_object, GraphQLInputObject, ScalarValue, FieldResult};

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// object macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.



#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = super::Context,
)]
impl super::Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    fn human(context: &super::Context, id: String) -> FieldResult<Human> {
        // Get a db connection.
        let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        let human = connection.find_human(&id)?;
        // Return the result.
        Ok(human)
    }
}

// Now, we do the same for our Mutation type.


#[graphql_object(
    context = super::Context,

    // If we need to use `ScalarValue` parametrization explicitly somewhere
    // in the object definition (like here in `FieldResult`), we should
    // declare an explicit type parameter for that, and specify it.
    scalar = S,
)]
impl<S: ScalarValue + Display> super::Mutation {
    fn createHuman(context: &super::Context, new_human: NewHuman) -> FieldResult<Human, S> {
        let db = context.pool.get_connection().map_err(|e| e.map_scalar_value())?;
        let human = db.insert_human(&new_human).map_err(|e| e.map_scalar_value())?;
        Ok(human)
    }
}
