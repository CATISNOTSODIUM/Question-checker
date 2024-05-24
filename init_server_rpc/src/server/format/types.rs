use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum QuestionType {
    MCQ, TFQ, CAT, ERC, ECS
}