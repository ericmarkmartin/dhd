use std::{convert::TryFrom, str::FromStr};

pub type Hash = i32;
pub struct HashList(Vec<Hash>);

impl From<Vec<Hash>> for HashList {
    fn from(hashes: Vec<Hash>) -> Self {
        HashList(hashes)
    }
}

impl From<HashList> for Vec<Hash> {
    fn from(hashlist: HashList) -> Self {
        hashlist.0
    }
}

impl From<HashList> for Vec<i64> {
    fn from(hashlist: HashList) -> Self {
        Vec::<Hash>::from(hashlist).iter().map(|x: &Hash| *x as i64).collect::<Vec<i64>>()
    }
}

impl TryFrom<&str> for HashList {
    type Error = <Hash as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.split("\n")
            .map(|s| s.parse::<Hash>())
            .collect::<Result<Vec<Hash>, _>>()
            .map(|hashes| hashes.into())
    }
}
