use std::fs::File;
use std::io::Read;

use crate::server::format::{
    check_question_id_format,
    question_type::{MyQuestion, MyBareQuestion}
};



pub async fn send_question(file_path: &String,question_id: &String, )-> Result<String, Box<dyn std::error::Error>> {
    //check the received question id
    check_question_id_format(&question_id)?;
    let result = fetch_question(&file_path,&question_id).await?;
    Ok(format!("{:#?}", result))
}





//File path example: ABC-123/4/6/Exercise1.md
pub async fn fetch_question(file_path: &String, question_id: &String) -> Result<MyQuestion, Box<dyn std::error::Error>> {
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