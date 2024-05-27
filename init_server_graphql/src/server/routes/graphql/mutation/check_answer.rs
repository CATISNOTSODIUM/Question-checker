use std::{fs::File, io::Read};
use serde_json;
use async_graphql::{Context, Object, Result, SimpleObject};

pub use crate::server::routes::graphql::response_types::*;
use crate::server::routes::graphql::types::User;

//Hash set
use std::collections::HashSet;

#[derive(Default)]
pub struct CheckAnswer;




#[Object]
impl CheckAnswer {
    //simple return CORRECT/WRONG for each question
    pub async fn check_answer
    (&self,_ctx: &Context<'_>,user: User, file_path: String, payload: MyResponse) -> Result<String>
    { 
        let result:Result<bool> = handler(&file_path,payload)
        .await.map_err(|e| format!("{}",e).into());
        
        // Inform client 
        match result {
            Ok(true) => Ok(format!("{}: CORRECT", user.display_name)),
            Ok(false) => Ok(format!("{}: WRONG", user.display_name)),
            Err(e) => Err(e),
        }
    }
    
    
    // Check answers of all questions in the file
    // For submission
    pub async fn check_answers 
    (&self, _ctx: &Context<'_>, _user: User, file_path: String, payloads: Vec<MyResponse>) -> Result<ScoreReport>
    { 
        //Find number of questions inside the file
        let answers_list_tmp:Result<Vec<MyResponse>> = fetch_answer(&file_path)
        .await.map_err(|e| format!("{}",e).into());

        let answers_list = answers_list_tmp.unwrap();
        let _number_question = answers_list.clone().len() as u32;
        let mut question_id_set: HashSet<String> = answers_list
        .iter()
        .map(|answer| answer.question_id.clone())
        .collect();
        

        // Rough check
        let mut correct_answer: Vec<String> = vec![];
        let mut wrong_answer: Vec<String> = vec![];
        let mut failed_submission_answer: Vec<String> = vec![]; // Invalid response 

        for payload in payloads{
            let question_id = payload.clone().question_id.clone();
            //remove question id from question_id_set
            question_id_set.remove(&question_id.clone());

            let result:Result<bool> = handler(&file_path,payload)
            .await.map_err(|e| format!("{}",e).into());
            match result {
                Ok(true) => {correct_answer.push(question_id);},
                Ok(false) => {wrong_answer.push(question_id);},
                Err(_) => {failed_submission_answer.push(question_id);},
            }
        };

        // check the remaining unanswer question
        let unanswered: Vec<String> = question_id_set.into_iter().collect();
        // Check if all questions are answered

        //TODO: Add progress to database for user: User 
        //To be implemented
        if unanswered.len() == 0 { //All questions have been answered.
            //add data to database
        }

        // Send report back to user
        Ok(ScoreReport {
            correct: correct_answer,
            wrong: wrong_answer,
            unanswered: unanswered,
            failed: failed_submission_answer
        })

    }
}



#[derive(SimpleObject)]
pub struct ScoreReport{
    correct: Vec<String>,
    wrong: Vec<String>,
    unanswered: Vec<String>,
    failed: Vec<String>,
} 


// Check answer from the desired file path
// file path example ABC-123/4/5/E xercise.md
pub async fn handler(file_path: &String, payload: MyResponse) -> Result<bool, Box<dyn std::error::Error>>  {
     //Struct validation: convert MyBareResponse -> MyResposne
    let response = payload;
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