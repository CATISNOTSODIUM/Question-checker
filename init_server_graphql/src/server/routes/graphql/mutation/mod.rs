mod check_answer;


//import queries
pub use check_answer::CheckAnswer;

#[derive(async_graphql::MergedObject, Default)]
pub struct MutationRoot(CheckAnswer);