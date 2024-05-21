use std::fs::File;
use std::io::Read;

//internal library
use super::response_type::{MyBareResponse, MyResponse, MyBody, QuestionType};


pub fn convert_format(input: &MyBareResponse) -> Result<MyResponse, Box<dyn std::error::Error>> {

    //Check question-id format
    let parts: Vec<_> = input.question_id.split('_').collect();
    if parts.len() != 4{
        return Err("Invalid question id")?;
    }
    
    match parts[1].parse::<u32>() {
        Ok(_) => (),
        Err(_) => {return Err("Invalid skillset id")?;},
    }

    match parts[2].parse::<u32>() {
        Ok(_) => (),
        Err(_) => {return Err("Invalid skill id")?;},
    }

    //check question type

    let question_type_list_string = vec!["MCQ".to_string(), "TFQ".to_string(), "CAT".to_string(), "ERC".to_string()];
    let question_type_list = vec![QuestionType::MCQ, QuestionType::TFQ, QuestionType::CAT, QuestionType::ERC];
    let question_type_index = question_type_list_string.clone()
    .iter()
    .position(|r| r == input.question_type.as_str())
    .ok_or("Invalid question type. Available question types: MCQ, TFQ, CAT, ERC")?;


    Ok(MyResponse {
        question_id: input.question_id.clone(),
        question_type: question_type_list[question_type_index].clone(),
        body: MyBody {
            answer: input.body.answer.clone(),
            answer_index: input.body.answer_index.clone()
        }
    })
}


pub async fn get_answer(file_name:String) -> Result<Vec<MyResponse>, Box<dyn std::error::Error>> {
    let root = project_root::get_project_root()?;
    //read file .json
    let file_name = file_name;
    let target_dir = root.join("assets/questions").join(file_name.as_str());
    let mut file = File::open(target_dir)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let answers_list : Vec<MyResponse> = serde_json::from_str(data.as_str())?;
    
    Ok(answers_list)
}