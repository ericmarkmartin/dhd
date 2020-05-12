use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/hashlist_schema.json",
    query_path = "graphql/hashlist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct HashlistMutation;

#[allow(dead_code)]
struct HashList {
    hashes: Vec<i64>,
}
