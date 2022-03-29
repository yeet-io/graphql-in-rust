use std::{io, sync::Arc};
use actix_web::{ get, route, web::{self, Data}, App, HttpResponse, HttpServer, Responder };
use actix_cors::Cors;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use crate::schema::{create_schema, Schema};

mod schema;

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok().body(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<Schema>, request: web::Json<GraphQLRequest>) -> impl Responder {
    HttpResponse::Ok().json(request.execute(&schema, &()).await)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}