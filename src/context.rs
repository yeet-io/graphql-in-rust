use tokio_postgres::Client;

pub struct Context {
    pub db: Client,
}

impl juniper::Context for Context {}
