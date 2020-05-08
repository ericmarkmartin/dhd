use super::redis::RedisPool;
use dhd_core::Diff;
use juniper::{EmptyMutation, RootNode};

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
    fn diff_with_hashes() -> Vec<Diff> {
        vec![]
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}
