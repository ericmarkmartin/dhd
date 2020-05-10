use std::{convert::TryFrom, str::FromStr};

pub type Hash = u32;
pub struct HashList(Vec<Hash>);

impl From<Vec<Hash>> for HashList {
    fn from(hashes: Vec<Hash>) -> Self {
        HashList(hashes)
    }
}

impl From<HashList> for Vec<i64> {
    fn from(hashlist: HashList) -> Self {
        hashlist
            .0
            .iter()
            .map(|x: &Hash| *x as i32 as i64)
            .collect::<Vec<i64>>()
    }
}

impl From<HashList> for Vec<i32> {
    fn from(hashlist: HashList) -> Self {
        hashlist
            .0
            .iter()
            .map(|x: &Hash| *x as i32)
            .collect::<Vec<i32>>()
    }
}

impl TryFrom<&str> for HashList {
    type Error = <Hash as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.split('\n')
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<Hash>, _>>()
            .map(|hashes| hashes.into())
    }
}
