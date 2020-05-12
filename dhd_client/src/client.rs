use super::graphql::mutation::{hashlist_mutation, HashlistMutation};
use super::graphql::query::{hashlist_query, HashlistQuery};
use dhd_core::hashlist::{Hash, HashList};
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use thiserror::Error;
use url::{self, Url};

#[derive(Error, Debug)]
pub enum DhdClientError {
    #[error("URL Parse Error")]
    UrlError(#[from] url::ParseError),
    #[error("Network Error")]
    RequestError(#[from] reqwest::Error),
    #[error("Query Error: {0}")]
    QueryError(String),
    #[error("Unknown Error")]
    UnknownError,
}
type DhdClientResult<T> = Result<T, DhdClientError>;

pub struct DhdClient {
    url: Url,
}

impl DhdClient {
    pub fn new(url: &str) -> DhdClientResult<DhdClient> {
        Ok(DhdClient {
            url: Url::parse(url)?,
        })
    }

    pub fn push(&self, username: &str, hashlist: HashList) -> DhdClientResult<String> {
        let body = HashlistMutation::build_query(hashlist_mutation::Variables {
            username: username.to_string(),
            hashlist: hashlist_mutation::HashList {
                hashes: hashlist.into(),
            },
        });
        let client = Client::new();
        let res = client.post(self.url.clone()).json(&body).send()?;

        let response: Response<hashlist_mutation::ResponseData> = res.json()?;
        if let Some(data) = response.data {
            Ok(data.commit_hashlist)
        } else {
            if let Some(errors) = response.errors {
                if errors.len() == 0 {
                    Err(DhdClientError::UnknownError)
                } else {
                    Err(DhdClientError::QueryError(errors[0].message.clone()))
                }
            } else {
                Err(DhdClientError::UnknownError)
            }
        }
    }

    pub fn pull(&self, username: &str) -> DhdClientResult<HashList> {
        let body = HashlistQuery::build_query(hashlist_query::Variables {
            username: username.to_string(),
        });
        let client = Client::new();
        let res = client.post(self.url.clone()).json(&body).send()?;

        let response: Response<hashlist_query::ResponseData> = res.json()?;
        if let Some(data) = response.data {
            Ok(data
                .get_hashlist
                .iter()
                .map(|x| *x as Hash)
                .collect::<Vec<Hash>>()
                .into())
        } else {
            if let Some(errors) = response.errors {
                if errors.len() == 0 {
                    Err(DhdClientError::UnknownError)
                } else {
                    Err(DhdClientError::QueryError(errors[0].message.clone()))
                }
            } else {
                Err(DhdClientError::UnknownError)
            }
        }
    }
}
