mod fetch_question;
mod fetch_content;

//import queries
pub use fetch_question::FetchQuestion;
pub use fetch_content::FetchContent;

#[derive(async_graphql::MergedObject, Default)]
pub struct QueryRoot(FetchQuestion, FetchContent);

