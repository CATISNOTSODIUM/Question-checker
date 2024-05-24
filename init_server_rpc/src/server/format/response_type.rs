use serde::{Deserialize, Serialize};
use crate::server::format::{check_question_id_format,get_question_type};
use crate::server::format::types::QuestionType;
use specta::Type;

//BARE FORMAT
//the formats are not checked.
#[derive(Deserialize, Debug,Type, Serialize)]
pub struct MyBareResponse { 
    pub question_id: String,
    pub body: MyBareResponseBody,
}

#[derive(Deserialize, Debug, Type,Serialize)]
pub struct MyBareResponseBody {
    pub answer: Vec<String>,
}

//actual data type


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyResponse { 
    pub question_id: String,
    pub body: MyBody,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyBody {
    pub answer: Vec<String>,
}

impl MyBareResponse {

    pub fn convert_format(&self) -> Result<MyResponse, Box<dyn std::error::Error>> {
        // Check question-id length
        check_question_id_format(&self.question_id)?;


        // parse MyBareResponse -> MyResponse
        Ok(MyResponse {
            question_id: self.question_id.clone(),
            body: MyBody {
                answer: self.body.answer.clone(),
            }
        })
    }
}

impl MyResponse {
    pub fn detailed_check(&self) -> Result<(),  Box<dyn std::error::Error>> {
        let question_type = get_question_type(&self.question_id)?;

        match question_type {
            QuestionType::MCQ => {
                //length of answer and answer index must be one
                if self.body.answer.len() != 1 {
                    return Err("MCQ Error: Length of answer and answer_index must be one.")?;
                }
            },
            QuestionType::TFQ => {

                //length of answer and answer index must be one
                if self.body.answer.len() != 1 {
                    return Err("TFQ Error: Length of answer and answer_index must be one.")?;
                }

                //check if the answer is true / false
                if self.body.answer[0] != "True".to_string() && self.body.answer[0] != "False".to_string(){
                    return Err("TFQ Error: The answer must be true or false.")?;
                }

            },
            QuestionType::CAT => {
                if self.body.answer.len() < 2 {
                    return Err("CAT Error: Length of answer and answer_index must be at least two.")?;
                } 
            },
            _ => {
                if self.body.answer.len() != 1 {
                    return Err("ECS/ERC Error: Length of answer and answer_index must be one.")?;
                }
            }
        }
        Ok(())
    }
}