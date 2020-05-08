#![feature(decl_macro, proc_macro_hygiene)]

use rocket::{response::content, State};
use std::sync::Arc;

use dhd_server::{
    redis::init_pool,
    schema::{create_schema, Context, Schema},
};

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    ctx: State<Arc<Context>>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Arc<Schema>>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &ctx)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    ctx: State<Arc<Context>>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Arc<Schema>>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &ctx)
}

fn main() {
    let pool = init_pool().unwrap();
    let schema = Arc::new(create_schema());
    let schema_context = Arc::new(Context::new(pool));

    rocket::ignite()
        .manage(schema_context.clone())
        .manage(schema.clone())
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
