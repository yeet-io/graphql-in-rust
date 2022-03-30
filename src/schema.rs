use juniper::GraphQLObject;
use crate::context::Context;
use juniper::{EmptySubscription, RootNode, FieldResult};

#[derive(GraphQLObject)]
#[graphql(description = "A person")]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    async fn person<'c>(context: &'c Context, id: i32) -> FieldResult<Option<Person>> {
        let row = context.db
            .query_one("SELECT id, name, age FROM person WHERE id = $1", &[&id])
            .await;
        if let Ok(person) = row {
            let id: i32 = person.get(0);
            let name: &str = person.get(1);
            let age: i32 = person.get(2);
            Ok(Some(Person {
                id,
                name: name.to_owned(),
                age,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    async fn create_person<'c>(context: &'c Context, name: String, age: i32) -> FieldResult<Option<Person>> {
        let row = context.db
            .query_one(
              "INSERT INTO person (name, age) VALUES ($1, $2) RETURNING id, name, age",
              &[&name, &age],
            )
            .await;
        if let Ok(person) = row {
            let id: i32 = person.get(0);
            let name: &str = person.get(1);
            let age: i32 = person.get(2);
            Ok(Some(Person {
                id,
                name: name.to_owned(),
                age,
            }))
        } else {
            Ok(None)
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::<Context>::new())
}
