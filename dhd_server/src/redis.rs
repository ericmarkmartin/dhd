use r2d2_redis::{
    r2d2::{self, Pool},
    redis::{self, Commands},
    RedisConnectionManager,
};

pub const HASHLIST_HASH_NAME: &str = "hashlists";
pub const HASHLIST_HASH_COUNTER_NAME: &str = "hashlists_ctr";

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn init_pool() -> Result<RedisPool, r2d2::Error> {
    let manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
    Pool::builder().build(manager)
}

pub fn lookup(mut conn: redis::Connection, id: String) -> Option<Vec<u32>> {
    match conn.hget("hashlists", &id) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub fn insert(mut conn: redis::Connection, id: String, hashes: Vec<u32>) -> Result<(), ()> {
    conn.hset("hashlists", &id, hashes).map_err(|_| ())
}
