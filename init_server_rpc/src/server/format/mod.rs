pub mod response_type;
pub mod question_type;
pub mod types;

use types::QuestionType;
//question id format [Question Type]_[skill-id]_[question-index]
pub fn check_question_id_format(input: &String) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn get_question_type(input: &String) -> Result<QuestionType, Box<dyn std::error::Error>> {
    let question_id_parts: Vec<_> = input.split('_').collect();
    if question_id_parts.len() != 3 {
        return Err("Invalid question id (format: [Question type]_[skill-id]_[question-index])")?;
    }

    let question_type_list_string = vec!["MCQ".to_string(), "TFQ".to_string(), "CAT".to_string(), "ERC".to_string()];
    let question_type_list = vec![QuestionType::MCQ, QuestionType::TFQ, QuestionType::CAT, QuestionType::ERC];
    let question_type_index = question_type_list_string.clone()
    .iter()
    .position(|r| r == question_id_parts[0])
    .ok_or("Invalid question type. Available question types: MCQ, TFQ, CAT, and ERC")?;

    let res = question_type_list[question_type_index].clone();
    Ok(res)
}