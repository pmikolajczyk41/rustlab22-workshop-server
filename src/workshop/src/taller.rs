use crate::swapi::SwapiClient;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

pub const YODA_HEIGHT: usize = 66;

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct YodaTallerOutcome {
    pub person: String,
    pub taller: bool,
}

#[derive(Debug, Error)]
pub enum YodaTallerError {
    #[error("Unexpected error")]
    UnexpectedError(reqwest::Error),
    #[error("Person not found")]
    PersonNotFound,
    #[error("Height not found")]
    HeightNotFound,
}

pub struct YodaTaller {
    pub client: SwapiClient,
}

impl YodaTaller {
    pub async fn is_taller_than(&self, name: &str) -> Result<YodaTallerOutcome, YodaTallerError> {
        let matches = self
            .client
            .people_by_name(name)
            .await
            .map_err(|e| YodaTallerError::UnexpectedError(e))?;
        if matches.is_empty() {
            return Err(YodaTallerError::PersonNotFound);
        }
        let person = matches[0].clone();
        let person_height =
            usize::from_str(&person.height).map_err(|_| YodaTallerError::HeightNotFound)?;
        Ok(YodaTallerOutcome {
            person: name.to_string(),
            taller: person_height < YODA_HEIGHT,
        })
    }
}
