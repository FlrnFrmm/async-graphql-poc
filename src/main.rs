mod models;
mod schema;
mod database;
mod routes;

use std::io::{Result, ErrorKind};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> Result<()> {
    let schema = schema::new().await
        .map_err(|e| std::io::Error::new(ErrorKind::ConnectionRefused, e.to_string()))?;

    HttpServer::new(move ||
        App::new()
            .data(schema.clone())
            .service(routes::graphiql)
            .service(routes::graphql))
        .bind("0.0.0.0:80")?
        .run()
        .await
}