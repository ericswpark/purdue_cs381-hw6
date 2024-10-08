use crate::structs::*;
use cs381_hw6::*;

use anyhow::Result;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

fn do_question_two(p: Vec<i32>, t: Vec<u32>, e: u16) -> Result<u32, AppError> {
    Ok(homework_points(&p, &t, e)?)
}

pub async fn question_two(Json(payload): Json<QuestionTwo>) -> impl IntoResponse {
    match do_question_two(payload.p, payload.t, payload.e) {
        Ok(result) => (StatusCode::OK, Json(QuestionTwoAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}
