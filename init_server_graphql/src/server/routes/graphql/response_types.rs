use serde::{Deserialize, Serialize};
use async_graphql::{InputObject, SimpleObject};

//BARE FORMAT
//the formats are not checked.
#[derive(InputObject)]
#[graphql(complex)]
#[derive(Deserialize, Debug, Default)]
pub struct MyBareResponse { 
    pub question_id: String,
    pub body: MyBareResponseBody,
}


#[derive(InputObject)]
#[derive(Deserialize, Debug, Default,Serialize)]
pub struct MyBareResponseBody {
    pub answer: Vec<String>,
}

//actual data type

#[derive(SimpleObject)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MyResponse { 
    pub question_id: String,
    pub body: MyBody,
}

#[derive(SimpleObject)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MyBody {
    pub answer: Vec<String>,
}


impl MyBareResponse {

    pub fn convert_format(&self) -> Result<MyResponse, Box<dyn std::error::Error>> {
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
    // Detailed check (check question, label, choices, ... ) will be later implemented.
    pub fn detailed_check(&self) -> Result<(),  Box<dyn std::error::Error>> {

        //get question type from first three characters
        let question_type = &(self.question_id.clone())[0..=2]; 

        match question_type {
            "MCQ" => {
                //length of answer and answer index must be one
                if self.body.answer.len() != 1 {
                    return Err("MCQ Error: Length of answer and answer_index must be one.")?;
                }
            },
            "TFQ" => {

                //length of answer and answer index must be one
                if self.body.answer.len() != 1 {
                    return Err("TFQ Error: Length of answer and answer_index must be one.")?;
                }

                //check if the answer is true / false
                if self.body.answer[0] != "True".to_string() && self.body.answer[0] != "False".to_string(){
                    return Err("TFQ Error: The answer must be true or false.")?;
                }

            },
            "CAT" => {
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