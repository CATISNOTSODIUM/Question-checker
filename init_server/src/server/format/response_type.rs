use serde::{Deserialize, Serialize};
use crate::server::format::check_question_id_format;
//BARE FORMAT
//the formats are not checked.
#[derive(Deserialize, Debug)]
pub struct MyBareResponse { 
    pub question_id: String,
    pub question_type: String,
    pub body: MyBareResponseBody,
}

#[derive(Deserialize, Debug)]
pub struct MyBareResponseBody {
    pub answer: Vec<String>,
    pub answer_index: Vec<u32>,
}

//actual data type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum QuestionType {
    MCQ, TFQ, CAT, ERC
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyResponse { 
    pub question_id: String,
    pub question_type: QuestionType,
    pub body: MyBody,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyBody {
    pub answer: Vec<String>,
    pub answer_index: Vec<u32>,
}

impl MyBareResponse {

    pub fn convert_format(&self) -> Result<MyResponse, Box<dyn std::error::Error>> {
        // Check question-id length
        check_question_id_format(&self.question_id)?;
        // check question type
        let question_type_list_string = vec!["MCQ".to_string(), "TFQ".to_string(), "CAT".to_string(), "ERC".to_string()];
        let question_type_list = vec![QuestionType::MCQ, QuestionType::TFQ, QuestionType::CAT, QuestionType::ERC];
        let question_type_index = question_type_list_string.clone()
        .iter()
        .position(|r| r == self.question_type.as_str())
        .ok_or("Invalid question type. Available question types: MCQ, TFQ, CAT, and ERC")?;
        // parse MyBareResponse -> MyResponse
        Ok(MyResponse {
            question_id: self.question_id.clone(),
            question_type: question_type_list[question_type_index].clone(),
            body: MyBody {
                answer: self.body.answer.clone(),
                answer_index: self.body.answer_index.clone()
            }
        })
    }
}

impl MyResponse {
    pub fn detailed_check(&self) -> Result<(),  Box<dyn std::error::Error>> {
 
        Ok(())
    }
}