//! Client to call the swapi API.

use std::time::Duration;

use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use tracing::instrument;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Query {
    results: Vec<Person>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Person {
    pub name: String,
    pub height: String,
}

pub struct SwapiClient {
    http_client: Client,
    base_url: String,
}

impl SwapiClient {
    pub fn new(base_url: String, timeout: Duration) -> anyhow::Result<Self> {
        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .context("can't build http client")?;
        Ok(Self {
            http_client,
            base_url,
        })
    }

    #[instrument(skip(self))]
    pub async fn people_by_name(&self, name: &str) -> Result<Vec<Person>, reqwest::Error> {
        let url = format!("{}/api/people/?search={name}", self.base_url);
        let query: Query = self
            .http_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(query.results)
    }
}
