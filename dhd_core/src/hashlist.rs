use itertools::Itertools;
use std::{convert::TryFrom, str::FromStr};

pub type Hash = i32;

#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct HashList {
    hashes: Vec<Hash>,
}

impl HashList {
    pub fn new(hashes: Vec<Hash>) -> Self {
        HashList { hashes }
    }

    pub fn to_delimited_string(&self) -> String {
        self.hashes
            .iter()
            .map(|&n| (n as u32).to_string())
            .join("\n")
    }
}

impl From<Vec<Hash>> for HashList {
    fn from(hashes: Vec<Hash>) -> Self {
        HashList::new(hashes)
    }
}

impl From<HashList> for Vec<Hash> {
    fn from(hashlist: HashList) -> Self {
        hashlist.hashes
    }
}

impl From<HashList> for Vec<i64> {
    fn from(hashlist: HashList) -> Self {
        hashlist.hashes.iter().map(|x| i64::from(*x)).collect()
    }
}

impl TryFrom<&str> for HashList {
    type Error = <Hash as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.split('\n')
            .map(|s| s.parse::<u32>().map(|x| x as Hash))
            .collect::<Result<Vec<Hash>, _>>()
            .map(|hashes| hashes.into())
    }
}
