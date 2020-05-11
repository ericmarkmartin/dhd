use super::redis::{RedisPool, HASHLIST_HASH_NAME};
use dhd_core::hashlist::{Hash, HashList};
use juniper::{FieldError, FieldResult, RootNode};
use r2d2_redis::redis::Commands;
use std::convert::TryFrom;

pub struct Context {
    pub db: RedisPool,
}

impl Context {
    pub fn new(db: RedisPool) -> Self {
        Self { db }
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context
)]
impl Query {
    fn get_hashlist(context: &Context, username: String) -> FieldResult<Vec<Hash>> {
        // TODO: Better mapping from Redis to client-facing GraphQL errors
        let mut conn = context.db.get()?;
        let hashlist_str: String = conn.hget("hashlists", username)?;

        HashList::try_from(hashlist_str.as_str())
            .map(|hashlist| hashlist.into())
            .map_err(|err| err.into())
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context
)]
impl Mutation {
    fn commit_hashlist(
        context: &Context,
        username: String,
        hashlist: HashList,
    ) -> Result<String, FieldError> {
        let mut conn = context.db.get()?;
        conn.hset(
            HASHLIST_HASH_NAME,
            &username,
            hashlist.to_delimited_string(),
        )?;
        Ok(username)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
