use serde::{Deserialize, Serialize};
use crate::server::format::check_question_id_format;

//QUESTION
#[derive(Deserialize, Debug)]
pub struct MyBareQuestion {
    pub question_id: String,
    pub question_type: String,
    pub body: MyBareQuestionBody,
}

#[derive(Deserialize, Debug)]
pub struct MyBareQuestionBody {
    pub question: String,
    pub label: Option<Vec<String>>,
    pub choices: Vec<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyQuestion { 
    pub question_id: String,
    pub question_type: QuestionType,
    pub body: MyQuestionBody,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyQuestionBody {
    pub question: String,
    pub label: Option<Vec<String>>,
    pub choices: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum QuestionType {
    MCQ, TFQ, CAT, ERC
}

impl MyBareQuestion {
    pub fn convert_format(&self) -> Result<MyQuestion, Box<dyn std::error::Error>> {
        //Rough check: check question id format & question type.
        // Check question-id length
        check_question_id_format(&self.question_id)?;
        // check question type
        let question_type_list_string = vec!["MCQ".to_string(), "TFQ".to_string(), "CAT".to_string(), "ERC".to_string()];
        let question_type_list = vec![QuestionType::MCQ, QuestionType::TFQ, QuestionType::CAT, QuestionType::ERC];
        let question_type_index = question_type_list_string.clone()
        .iter()
        .position(|r| r == self.question_type.as_str())
        .ok_or("Invalid question type. Available question types: MCQ, TFQ, CAT, and ERC")?;
        Ok(MyQuestion {
            question_id: self.question_id.clone(),
            question_type: question_type_list[question_type_index].clone(),
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
        
        match self.question_type {
            QuestionType::MCQ => {
                //label must be null
                if !self.body.label.is_none() {
                    return Err("MCQ Error: body.label must be null.")?;
                }
            },
            QuestionType::TFQ => {
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
            QuestionType::CAT => {
                if self.body.label.is_none() {
                    return Err("CAT Error: body.label cannot be empty.")?;
                } 
                let tmp_label = (self.body.label.clone()).unwrap();
                if tmp_label.len() < 2 {
                    return Err("CAT Error: body.label must have at least two elements")?;
                }
                
            },
            QuestionType::ERC => {
                //not yet implemented
            }
        }
        Ok(())
    }
}