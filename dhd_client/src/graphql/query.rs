use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/hashlist_schema.json",
    query_path = "graphql/hashlist_query.graphql",
    response_derives = "Debug"
)]
pub struct HashlistQuery;
