use std::time::Duration;

use reqwest::{Client, ClientBuilder, Error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub height: String,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    pub results: Vec<Person>,
}

#[derive(Clone)]
pub struct SwapiClient {
    client: Client,
    base_url: String,
}

impl SwapiClient {
    fn search_url(&self, arg: &str) -> String {
        format!("{}/api/people/?search={}", self.base_url, arg)
    }

    pub async fn people_by_name(&self, name: &str) -> Result<Vec<Person>, Error> {
        let url = self.search_url(name);
        let result: SearchResult = self.client.get(url).send().await?.json().await?;
        Ok(result.results)
    }
}

impl SwapiClient {
    pub fn new(base_url: String, timeout: Duration) -> anyhow::Result<Self> {
        let client = ClientBuilder::new().timeout(timeout).build()?;
        Ok(SwapiClient { client, base_url })
    }
}
