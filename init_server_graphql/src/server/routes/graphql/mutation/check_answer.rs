use std::{fs::File, io::Read};
use serde_json;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject};

pub use crate::server::routes::graphql::response_types::*;
use crate::server::routes::graphql::types::User;

#[derive(Default)]
pub struct CheckAnswer;




#[Object]
impl CheckAnswer {
    //simple return CORRECT/WRONG for each question
    pub async fn check_answer
    (&self, user: User, file_path: String, payload: MyBareResponse) -> Result<String>
    { 
        let result:Result<bool> = handler(&file_path,payload)
        .await.map_err(|e| format!("{}",e).into());

        //Add progress to database for user: User 
        //To be implemented


        
        // Inform client 
        match result {
            Ok(true) => Ok(format!("{}: CORRECT", user.display_name)),
            Ok(false) => Ok(format!("{}: WRONG", user.display_name)),
            Err(e) => Err(e),
        }
    }
    pub async fn check_answers // For submission
    (&self, user: User, file_path: String, payloads: Vec<MyBareResponse>) -> Result<ScoreReport>
    { 
        //Find number of questions inside the file
        let answers_list:Result<Vec<MyResponse>> = fetch_answer(&file_path)
        .await.map_err(|e| format!("{}",e).into());

        let number_question: u32 = answers_list.unwrap().len() as u32;
        // Rough check
        let mut correct_answer = 0;
        let mut wrong_answer = 0;
        let mut failed_submission_answer = 0; // Cannot check 

        for payload in payloads{
            let result:Result<bool> = handler(&file_path,payload)
            .await.map_err(|e| format!("{}",e).into());
            match result {
                Ok(true) => {correct_answer+=1;},
                Ok(false) => {wrong_answer+=1;},
                Err(_) => {failed_submission_answer+=1;},
            }
        };

        // Check if all questions are answered
        let unanswered_count = number_question - correct_answer - wrong_answer - failed_submission_answer;
        //Add progress to database for user: User 
        //To be implemented


        // Send report back to user
        Ok(ScoreReport {
            correct: correct_answer,
            wrong: wrong_answer,
            unanswered: unanswered_count,
            failed: failed_submission_answer
        })

    }


}

#[derive(SimpleObject)]
pub struct ScoreReport{
    correct: u32,
    wrong: u32,
    unanswered: u32,
    failed: u32,
} 

//file path example ABC-123/4/5/E xercise.md
pub async fn handler(file_path: &String, payload: MyBareResponse) -> Result<bool, Box<dyn std::error::Error>>  {
     //Struct validation: convert MyBareResponse -> MyResposne
    let response = payload.convert_format()?;
    //detailed check
    response.detailed_check()?;

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
        Ok(true)
    } else {
        Ok(false)
    }
    
}


pub async fn fetch_answer(file_path:& String) -> Result<Vec<MyResponse>, Box<dyn std::error::Error>> {
    let root = project_root::get_project_root()?;
    let target_dir = root.join("assets/materials").join(file_path.as_str());
    let mut file = File::open(target_dir)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let answers_list : Vec<MyResponse> = serde_json::from_str(data.as_str())?;
    
    Ok(answers_list)
}