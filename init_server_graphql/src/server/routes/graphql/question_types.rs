//Written by Thitiwat Kosolpattanadurong
use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;


/*
Here is the structure of the question sent to frontend. 
{
    question_id: String
    body: {
        question: String (E.g. "What is the most commonly-found lattice structure of NaCl?")
        label: Option<Vec<String>> (List all possible categories (for CAT question). For other types, left this part as NULL.)
        choices: Vec<String> (All possible choices you want the user to answer. (E.g. In TFQ, choices=["True", "False"]))
    }
}
*/

#[derive(SimpleObject)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MyQuestion { 
    pub question_id: String,
    pub body: MyQuestionBody,
}


#[derive(SimpleObject)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MyQuestionBody {
    pub question: String,
    pub label: Option<Vec<String>>,
    pub choices: Vec<String>
}


// Error handling for invalid responses.
impl MyQuestion {
    pub fn detailed_check(&self) -> Result<(),  Box<dyn std::error::Error>> {

        //choice must not be empty.
        let tmp_choice = self.body.choices.clone();
        if tmp_choice.len() < 2 {
            return Err("Error: the question must have at least two choices.")?;
        }

        //get question type from first three characters
        let question_type = &(self.question_id.clone())[0..=2]; 

        match question_type {
            "MCQ" => {
                //label must be null
                if !self.body.label.is_none() {
                    return Err("MCQ Error: body.label must be null.")?;
                }
            },
            "TFQ" => {
                //label must be null
                if !self.body.label.is_none() {
                    return Err("TFQ Error: body.label must be null.")?;
                }
                //choice must be True, False 
                if self.body.choices !=  ["False".to_owned(), "True".to_owned()] &&
                    self.body.choices !=  ["True".to_owned(), "False".to_owned()] {
                    return Err("TFQ Error: body.choices must be [\"True\", \"False\"]")?;
                }
            },
            "CAT" => {
                if self.body.label.is_none() {
                    return Err("CAT Error: body.label cannot be empty.")?;
                } 
                let tmp_label = (self.body.label.clone()).unwrap();
                if tmp_label.len() < 2 {
                    return Err("CAT Error: body.label must have at least two elements")?;
                }
                
            },
            _ => {
                //not yet implemented
            }
        }
        Ok(())
    }
}