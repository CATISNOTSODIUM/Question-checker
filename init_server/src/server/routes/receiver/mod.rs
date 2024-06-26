pub mod handler;

use axum::{Json,
    response::IntoResponse,
    http::StatusCode};
use crate::server::format::response_type::MyBareResponse;
use handler::fetch_answer;


pub async fn check_answer(
//parse JSON input into struct MyBareResponse
Json(payload): Json<MyBareResponse>,) -> impl IntoResponse
{ 
   match handler(payload).await {
    //Successful response
    Ok(res) => (StatusCode::OK, res.to_owned()).into_response(),
    //For unsuccessful response
    Err(err) => {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Process failed: {}", &err)).into_response();
    }
   }
}

pub async fn handler(payload: MyBareResponse) -> Result<String, Box<dyn std::error::Error>>  {
     //Struct validation: convert MyBareResponse -> MyResposne
    let response = payload.convert_format()?;
    //detailed check
    response.detailed_check()?;
    //retrieve the answer key (which is a list of all questions)
    let answers_list = fetch_answer("question.json".to_owned()).await?;

    //Brute force finding the question that has the same id with response.  (which is bad)

    let target_answer_index = answers_list.iter()
    .position(|ans| ans.question_id == response.clone().question_id)
    .ok_or("Cannot find the actual answer from database.")?; 

    let target_answer = &answers_list[target_answer_index];

    //check question type
    if target_answer.clone().question_type !=  response.clone().question_type{
        return Err(format!("According to the question-id, the expected question type for this response \
        format is not the same as the question type identified in the database. \
        Got {:?} instead of {:?}.", 
        &response.question_type, &target_answer.question_type))?;
    }

    //check answer
    if target_answer.clone().body.answer == response.clone().body.answer
    && target_answer.clone().body.answer_index ==  response.clone().body.answer_index
    {
        Ok("CORRECT".to_string())
    } else {
        Ok("WRONG".to_string())
    }
}
