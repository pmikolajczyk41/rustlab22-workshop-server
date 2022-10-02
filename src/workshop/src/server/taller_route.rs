use std::sync::Arc;

use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::taller::{YodaTaller, YodaTallerError, YodaTallerOutcome};

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct YodaTallerResponse {
    pub query: String,
    #[serde(flatten)]
    pub result: YodaTallerOutcome,
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct ErrorBody {
    pub query: String,
    pub error: String,
}

pub struct YodaTallerErrorResponse {
    pub query: String,
    pub error: YodaTallerError,
}

impl IntoResponse for YodaTallerErrorResponse {
    fn into_response(self) -> Response {
        match self.error {
            YodaTallerError::PersonNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    query: self.query,
                    error: "Person not found".to_string(),
                }),
            ),
            YodaTallerError::HeightNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    query: self.query,
                    error: "Person's height is unknown".to_string(),
                }),
            ),
            YodaTallerError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody {
                    query: self.query,
                    error: "Unexpected error".to_string(),
                }),
            ),
        }
        .into_response()
    }
}

pub async fn taller(
    Path(name): Path<String>,
    Extension(yoda_taller): Extension<Arc<YodaTaller>>,
) -> Result<Json<YodaTallerResponse>, YodaTallerErrorResponse> {
    match yoda_taller.is_taller_than(&name).await {
        Ok(outcome) => Ok(YodaTallerResponse {
            query: name,
            result: outcome,
        }
        .into()),
        Err(e) => Err(YodaTallerErrorResponse {
            query: name,
            error: e,
        }),
    }
}
