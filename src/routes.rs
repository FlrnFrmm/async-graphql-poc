use actix_web::{get, route, web, Error, HttpResponse};
use juniper_actix::{graphiql_handler, graphql_handler};

use crate::schema::{Schema, Context};

#[get("/")]
pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(req: actix_web::HttpRequest, payload: actix_web::web::Payload, schema: web::Data<Schema>) -> Result<HttpResponse, Error> {
    let context = Context::new();
    println!("I'm here for a {}", req.method());
    graphql_handler(&schema, &context, req, payload).await
}