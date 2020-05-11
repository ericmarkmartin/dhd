use dhd_core::hashlist::{Hash, HashList};
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use thiserror::Error;
use url::{self, Url};

#[derive(Error, Debug)]
pub enum DhdClientError {
    #[error("URL Parse Error")]
    UrlError(#[from] url::ParseError),
    #[error("Network Error")]
    RequestError(#[from] reqwest::Error),
    #[error("Query Error")]
    QueryError,
    #[error("Data Format Error")]
    DataFormatError(#[from] std::num::ParseIntError),
}
type DhdClientResult<T> = Result<T, DhdClientError>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/hashlist_schema.json",
    query_path = "graphql/hashlist_query.graphql",
    response_derives = "Debug"
)]
pub struct HashlistQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/hashlist_schema.json",
    query_path = "graphql/hashlist_mutation.graphql",
    response_derives = "Debug"
)]
pub struct HashlistMutation;

pub struct DhdClient {
    url: Url,
}

impl DhdClient {
    pub fn new(url: &str) -> DhdClientResult<DhdClient> {
        Ok(DhdClient {
            url: Url::parse(url)?,
        })
    }

    pub fn push(&self, hashlist: HashList) -> DhdClientResult<u32> {
        let processed_list: Vec<i64> = Vec::<Hash>::from(hashlist)
            .iter()
            .map(|x| u32::from(*x) as i32 as i64)
            .collect();
        let body = HashlistMutation::build_query(hashlist_mutation::Variables {
            hashlist: processed_list,
        });
        let client = reqwest::blocking::Client::new();
        let res = client.post(self.url.clone()).json(&body).send()?;

        let response: Response<hashlist_mutation::ResponseData> = res.json()?;
        if let Some(data) = response.data {
            Ok(data.create_hashlist.parse::<u32>()?)
        } else {
            println!("{:?}", response);
            Err(DhdClientError::QueryError)
        }
    }

    pub fn pull(&self, id: u32) -> DhdClientResult<HashList> {
        let body = HashlistQuery::build_query(hashlist_query::Variables {
            hashlist_id: id.to_string(),
        });
        let client = reqwest::blocking::Client::new();
        let res = client.post(self.url.clone()).json(&body).send()?;

        let response: Response<hashlist_query::ResponseData> = res.json()?;
        if let Some(data) = response.data {
            Ok(data
                .get_hashlist
                .iter()
                .map(|x| Hash::from(*x as u32))
                .collect::<Vec<Hash>>()
                .into())
        } else {
            Err(DhdClientError::QueryError)
        }
    }
}
