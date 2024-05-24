use std::{fs::File, io::Read};
use serde_json;
use async_graphql::{ComplexObject,Result};

pub use crate::server::routes::graphql::question_types::*;


#[ComplexObject]
impl MyBareQuestion {
    pub async fn get_question(&self, file_path: String,question_id: String) -> Result<MyQuestion> {
        
        // Get MyQuestion
        let question = fetch_question(&file_path,&question_id)
        .await
        .map_err(|e| format!("{}",e).into());


        question
    }
    pub async fn get_questions(&self, file_path: String) -> Result<Vec<MyQuestion>> {
        fetch_questions(&file_path).await
        .map_err(|e| format!("{}",e).into())
    }
}






//Handlers

pub async fn fetch_question(file_path: &String, question_id: &String) -> Result<MyQuestion, Box<dyn std::error::Error>> {
    //check question id format
    check_question_id_format(&question_id).await?;
    
    let root = project_root::get_project_root()?;
    //read file .json
    let target_dir = root.join("assets/materials").join(file_path.as_str());
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

//Fetch every questions in the file
pub async fn fetch_questions(file_path: &String) -> Result<Vec<MyQuestion>, Box<dyn std::error::Error>> {

    let root = project_root::get_project_root()?;
    //read file .json
    let target_dir = root.join("assets/materials").join(file_path.as_str());
    let mut file = File::open(target_dir)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let question_list : Vec<MyBareQuestion> = serde_json::from_str(data.as_str())?;

    // Question list
    let mut response: Vec<MyQuestion> = Vec::new();
    for my_bare_question in question_list {
        // First step: check the format of each question
        check_question_id_format(&my_bare_question.question_id).await?;
        let my_question = my_bare_question.clone().convert_format()?;
        my_question.detailed_check()?; // Final check
        response.push(my_question);
    }
    Ok(response)
}


pub async fn check_question_id_format(input: &String) -> Result<(), Box<dyn std::error::Error>> {
    let question_id_parts: Vec<_> = input.split('_').collect();
    if question_id_parts.len() != 3 {
        return Err("Invalid question id (format: [Question type]_[skill-id]_[question-index])")?;
    }

    let question_type_list_string = vec!["MCQ".to_string(), "TFQ".to_string(), "CAT".to_string(), "ERC".to_string()];
    let _question_type_index = question_type_list_string.clone()
    .iter()
    .position(|r| r == question_id_parts[0])
    .ok_or("Invalid question type. Available question types: MCQ, TFQ, CAT, and ERC")?;

    // check skill id
    match question_id_parts[1].parse::<u32>() {
        Ok(_) => (),
        Err(_) => return Err("Invalid skill id. Skill id must be in u32.")?,
    }


    Ok(())
}