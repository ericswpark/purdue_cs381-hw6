use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize)]
pub struct QuestionTwo {
    pub(crate) b: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionTwoAnswer {
    pub(crate) answer: u32,
}

#[derive(Deserialize)]
pub struct QuestionThree {
    pub(crate) c: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionThreeAnswer {
    pub(crate) answer: u32,
}

#[derive(Deserialize)]
pub struct QuestionThreeCorrected {
    pub(crate) sand_dunes: Vec<u32>,
    pub(crate) cost: Vec<Vec<Vec<u32>>>,
}

#[derive(Error, Debug)]
pub enum QuestionThreeError {
    #[error("Cost array dimensions are wrong!")]
    CostArrayDimensionsIncorrect,
}

impl IntoResponse for QuestionThreeError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            QuestionThreeError::CostArrayDimensionsIncorrect => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
        };
        (status, Json(serde_json::json!({ "error": body }))).into_response()
    }
}

#[derive(Deserialize)]
pub struct QuestionFour {
    pub(crate) a: Vec<String>,
    pub(crate) m: u32,
}

#[derive(Serialize)]
pub struct QuestionFourAnswer {
    pub(crate) answer: u32,
}
