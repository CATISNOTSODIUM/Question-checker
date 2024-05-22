use axum::{response::IntoResponse,http::StatusCode, extract::Query};
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use crate::server::format::{
    check_question_id_format,
    question_type::{MyQuestion, MyBareQuestion}
};


#[derive(Deserialize)]
pub struct QueryExtractor {
    question_id: String
}

pub async fn send_question(payload: Query<QueryExtractor>)-> impl IntoResponse {
    //check the received question id
    match check_question_id_format(&payload.question_id) {
        Ok(_) => (),
        Err(err) => {return (StatusCode::INTERNAL_SERVER_ERROR, format!("Process failed: {}", &err)).into_response();}
    }
    match fetch_question("question.json".to_owned(),&payload.question_id).await {
        Ok(res) => (StatusCode::OK ,format!("{:#?}",res)).into_response(),
        Err(err) => {return (StatusCode::INTERNAL_SERVER_ERROR, format!("Process failed (question-id {}): {}",&payload.question_id,&err)).into_response();}
    }
}






pub async fn fetch_question(file_name:String, question_id: &String) -> Result<MyQuestion, Box<dyn std::error::Error>> {
    let root = project_root::get_project_root()?;
    //read file .json
    let file_name = file_name;
    let target_dir = root.join("assets/questions").join(file_name.as_str());
    let mut file = File::open(target_dir)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let answers_list : Vec<MyBareQuestion> = serde_json::from_str(data.as_str())?;


    let target_answer_index = answers_list.iter()
    .position(|ans| ans.question_id == question_id.clone())
    .ok_or("Cannot find question id from database.")?; 
    
    let response = answers_list[target_answer_index].convert_format()?;
    
    //check the question format based on question types.
    response.detailed_check()?;
    Ok(response)
}