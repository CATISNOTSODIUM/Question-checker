pub mod response_type;
pub mod handler;

use axum::Json;
use response_type::MyBareResponse;
use handler::{convert_format, get_answer};


pub async fn check_answer(
//parse JSON input into struct MyBareResponse
Json(payload): Json<MyBareResponse>,) -> String 
{ 
    //Struct validation: convert MyBareResponse -> MyResposne
    let response = convert_format(&payload).unwrap();
    
    //retrieve the answer key (which is a list of all questions)
    let answers_list = get_answer("answer.json".to_owned()).await;

    //Brute force finding the question that has the same id with response.  (which is bad)

    let target_answer_index = answers_list.iter()
    .position(|ans| ans.question_id == response.clone().question_id)
    .expect("cannot find answer"); 

    let target_answer = &answers_list[target_answer_index];

    //check question type
    if target_answer.clone().question_type !=  response.clone().question_type{
        return "INVALID question TYPE".to_string();
    }

    //check answer
    if target_answer.clone().body.answer == response.clone().body.answer
    && target_answer.clone().body.answer_index ==  response.clone().body.answer_index
    {
        "CORRECT".to_string()
    } else {
        "WRONG".to_string()
    }

}


