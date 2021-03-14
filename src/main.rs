mod models;
mod schema;
mod database;
mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
            .data(schema::new())
            .service(routes::graphiql)
            .service(routes::graphql))
        .bind("0.0.0.0:80")?
        .run()
        .await
}