use serde::{Deserialize, Serialize};
//BARE FORMAT
//the formats are not checked.
#[derive(Deserialize, Debug)]
pub struct MyBareResponse { 
    pub question_id: String,
    pub question_type: String,
    pub body: MyBareBody,
}

#[derive(Deserialize, Debug)]
pub struct MyBareBody {
    pub answer: String,
    pub answer_index: u32,
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
    pub answer: String,
    pub answer_index: u32,
}

