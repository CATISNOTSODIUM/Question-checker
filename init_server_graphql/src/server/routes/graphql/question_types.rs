use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

//QUERY QUESTION
//RAW DATA FROM DATABASE
#[derive(SimpleObject)]
#[graphql(complex)]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct MyBareQuestion {
    pub question_id: String,
    pub body: MyBareQuestionBody,
}

#[derive(SimpleObject)]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct MyBareQuestionBody {
    pub question: String,
    pub label: Option<Vec<String>>,
    pub choices: Vec<String>
}


//FINALIZED VERSION
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


//Formatting
impl MyBareQuestion {
    pub fn convert_format(&self) -> Result<MyQuestion, Box<dyn std::error::Error>> {

        Ok(MyQuestion {
            question_id: self.question_id.clone(),
            body: MyQuestionBody {
                question: self.body.question.clone(),
                label: self.body.label.clone(),
                choices: self.body.choices.clone(),
            }
        })

    }
}


impl MyQuestion {
    // Detailed check (check question, label, choices, ... ) will be later implemented.
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