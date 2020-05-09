use super::redis::RedisPool;
use dhd_core::hashlist::{Hash, HashList};
use itertools::Itertools;
use juniper::{FieldError, RootNode};
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
    fn get_hashlist(context: &Context, id: String) -> Result<Vec<Hash>, FieldError> {
        // TODO: Better mapping from Redis to client-facing GraphQL errors
        let mut conn = context.db.get()?;
        let hashlist_str: String = conn.hget("hashlists", id.parse::<u32>()?)?;

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
    fn create_hashlist(context: &Context, hashlist: Vec<i32>) -> Result<String, FieldError> {
        let mut conn = context.db.get()?;
        let s = hashlist.iter().map(|&n| (n as u32).to_string()).join("\n");
        let ctr: u32 = conn.incr::<_, _, u32>("hashlist_ctr", 1)? - 1;
        conn.hset("hashlists", ctr, s)?;
        Ok(ctr.to_string())
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
