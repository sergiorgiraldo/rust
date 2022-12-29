#[macro_use] extern crate rocket;

use std::time::Instant;
use juniper_rocket;
use rocket::{response::content, State};
use juniper::EmptySubscription;
mod db;
mod models;
mod schema;

use db::DataContext;
use schema::*;

#[get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[get("/grapqhl?<request>")]    
fn get_graphql_handler(
    context: &State<DataContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}


#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<DataContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    let start = Instant::now();
    let response = request.execute_sync(&schema, &context);
    println!("Request took {:?}", start.elapsed());
    response
}

#[launch]
fn rocket() -> _ {
        rocket::build()
        .manage(DataContext::init().unwrap())
        .manage(Schema::new(Query, Mutation, EmptySubscription::<DataContext>::new()))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
