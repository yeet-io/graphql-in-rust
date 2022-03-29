use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A person")]
struct Person {
    name: String,
    age: i32,
}

use juniper::FieldResult;
pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn person(_name: String) -> FieldResult<Person> {
        Ok(Person {
            name: "Bob".to_string(),
            age: 42,
        })
    }
}

use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}