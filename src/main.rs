use std::{io, sync::Arc};
use actix_web::{ get, route, web::{self, Data}, App, HttpResponse, HttpServer, Responder };
use actix_cors::Cors;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use tokio_postgres::NoTls;
use crate::schema::{create_schema, Schema};
use crate::context::Context;

mod schema;
mod context;

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok().body(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<Schema>, context: web::Data<Context>, request: web::Json<GraphQLRequest>) -> impl Responder {
    HttpResponse::Ok().json(request.execute(&schema, &context).await)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (client, connection) =
        tokio_postgres::connect("postgresql://username:password@localhost/database", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let schema = Arc::new(create_schema());

    let context = Arc::new(Context {
        db: client,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(Data::from(context.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}