use rocket_contrib::databases::redis::{self, Commands};

#[rocket_contrib::database("dhd_db")]
pub struct DhdDbConn(redis::Connection);

pub fn lookup(conn: DhdDbConn, id: String) -> Option<Vec<u32>> {
    match conn.hget("hashlists", &id) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub fn insert(conn: DhdDbConn, id: String, hashes: Vec<u32>) -> Result<(), ()> {
    conn.hset("hashlists", &id, hashes).map_err(|_| ())
}
