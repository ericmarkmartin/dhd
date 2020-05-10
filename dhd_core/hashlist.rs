use std::{convert::TryFrom, str::FromStr};

#[derive(Copy, Clone)]
pub struct Hash(u32);

pub struct HashList(Vec<Hash>);

impl From<Vec<Hash>> for HashList {
    fn from(hashes: Vec<Hash>) -> Self {
        HashList(hashes)
    }
}

impl From<u32> for Hash {
    fn from(u: u32) -> Self {
        Hash(u)
    }
}

impl<T> From<HashList> for Vec<T>
where
    T: From<Hash>,
{
    fn from(hashlist: HashList) -> Self {
        hashlist
            .0
            .iter()
            .map(|x: &Hash| T::from(*x))
            .collect::<Vec<T>>()
    }
}

impl From<HashList> for Vec<i32> {
    fn from(hashlist: HashList) -> Self {
        hashlist
            .0.iter().map(|x: &Hash| x.0 as i32).collect()
    }
}

impl FromStr for Hash {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).map(Hash::from)
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
