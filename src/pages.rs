use crate::structs::*;
use cs381_hw5::*;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

fn do_question_two(b: Vec<u32>) -> Result<u32, ()> {
    let result = valid_tours(&b);
    Ok(result)
}
pub async fn question_two(Json(payload): Json<QuestionTwo>) -> impl IntoResponse {
    match do_question_two(payload.b) {
        Ok(result) => (StatusCode::OK, Json(QuestionTwoAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

fn do_question_three_a(c: Vec<u32>) -> Result<u32, ()> {
    let result = sand_dunes_merging(&c);
    Ok(result)
}
pub async fn question_three_a(Json(payload): Json<QuestionThree>) -> impl IntoResponse {
    match do_question_three_a(payload.c) {
        Ok(result) => {
            (StatusCode::OK, Json(QuestionThreeAnswer { answer: result })).into_response()
        }
        Err(e) => e.into_response(),
    }
}

fn do_question_three_a_corrected(
    sand_dunes: Vec<u32>,
    cost: Vec<Vec<Vec<u32>>>,
) -> Result<u32, QuestionThreeError> {
    // Check dimensions of array
    if cost.len() < sand_dunes.len() {
        return Err(QuestionThreeError::CostArrayDimensionsIncorrect);
    }

    for i in cost.iter() {
        if i.len() < sand_dunes.len() {
            return Err(QuestionThreeError::CostArrayDimensionsIncorrect);
        }

        for j in i.iter() {
            if j.len() < sand_dunes.len() {
                return Err(QuestionThreeError::CostArrayDimensionsIncorrect);
            }
        }
    }

    let inner_sliced_cost: Vec<Vec<&[u32]>> = cost
        .iter()
        .map(|v_o| v_o.iter().map(|v_i| v_i.as_slice()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let sliced_cost: Vec<&[&[u32]]> = inner_sliced_cost
        .iter()
        .map(|v| v.as_slice())
        .collect::<Vec<_>>();

    let result = sand_dunes_arbitrary_cost_merging(&sand_dunes, &sliced_cost);
    Ok(result)
}

pub async fn question_three_a_corrected(
    Json(payload): Json<QuestionThreeCorrected>,
) -> impl IntoResponse {
    match do_question_three_a_corrected(payload.sand_dunes, payload.cost) {
        Ok(result) => {
            (StatusCode::OK, Json(QuestionThreeAnswer { answer: result })).into_response()
        }
        Err(e) => e.into_response(),
    }
}

fn do_question_three_b(c: Vec<u32>) -> Result<u32, ()> {
    let result = greedy_sand_dune_merging(&c);
    Ok(result)
}
pub async fn question_three_b(Json(payload): Json<QuestionThree>) -> impl IntoResponse {
    match do_question_three_b(payload.c) {
        Ok(result) => {
            (StatusCode::OK, Json(QuestionThreeAnswer { answer: result })).into_response()
        }
        Err(e) => e.into_response(),
    }
}

fn do_question_four(a: Vec<String>, m: u32) -> Result<u32, ()> {
    let result = word_wrapper(&a, m);
    Ok(result)
}
pub async fn question_four(Json(payload): Json<QuestionFour>) -> impl IntoResponse {
    match do_question_four(payload.a, payload.m) {
        Ok(result) => (StatusCode::OK, Json(QuestionFourAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}
