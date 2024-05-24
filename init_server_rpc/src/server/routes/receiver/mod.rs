pub mod handler;

use crate::server::format::response_type::MyBareResponse;
use handler::fetch_answer;


pub async fn check_answer(file_path: &String, payload: MyBareResponse) -> Result<String, Box<dyn std::error::Error>>
{ 
   match handler(&file_path,payload).await {
    //Successful response
    Ok(res) => Ok(res.to_owned()),
    //For unsuccessful response
    Err(err) => {
        Err(format!("Process failed: {}", &err))?
    }
   }
}

//file path example ABC-123/4/5/Exercise.md
pub async fn handler(file_path: &String, payload: MyBareResponse) -> Result<String, Box<dyn std::error::Error>>  {
     //Struct validation: convert MyBareResponse -> MyResposne
    let response = payload.convert_format()?;
    //detailed check
    response.detailed_check()?;
    //retrieve the answer key (which is a list of all questions)

    let root = project_root::get_project_root()?;
    //read file .json
    let answers_list = fetch_answer(&file_path).await?;

    //Brute force finding the question that has the same id with response.  (which is bad)

    let target_answer_index = answers_list.iter()
    .position(|ans| ans.question_id == response.clone().question_id)
    .ok_or("Cannot find the actual answer from database.")?; 

    let target_answer = &answers_list[target_answer_index];

    //check answer
    if target_answer.clone().body.answer == response.clone().body.answer
    {
        Ok("CORRECT".to_string())
    } else {
        Ok("WRONG".to_string())
    }
}
