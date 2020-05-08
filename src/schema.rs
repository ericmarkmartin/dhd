use super::redis::RedisPool;
use difference::Difference;
use juniper::{EmptyMutation, RootNode};

#[derive(juniper::GraphQLEnum)]
enum DiffType {
    Same,
    Add,
    Rem,
}

#[derive(juniper::GraphQLObject)]
struct Diff {
    diff_type: DiffType,
    data: String,
}

impl From<Difference> for Diff {
    fn from(difference: Difference) -> Self {
        use Difference::*;
        match difference {
            Same(data) => Diff {
                diff_type: DiffType::Same,
                data,
            },
            Add(data) => Diff {
                diff_type: DiffType::Add,
                data,
            },
            Rem(data) => Diff {
                diff_type: DiffType::Rem,
                data,
            },
        }
    }
}

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
