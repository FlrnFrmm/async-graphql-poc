use actix_web::{get, route, web::Data, Error, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

use crate::schema::Schema;

#[get("/")]
pub async fn graphiql() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql")
                .subscription_endpoint("/graphql")
    )))
}

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(schema: Data<Schema>, request: Request) -> Response {
    schema.execute(request.into_inner()).await.into()
}