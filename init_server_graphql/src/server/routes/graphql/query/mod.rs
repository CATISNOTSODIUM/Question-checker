mod fetch_question;
mod fetch_content;

use async_graphql::Object;
//import queries
pub use fetch_question::MyBareQuestion;
pub use fetch_content::FetchContent;

#[derive(async_graphql::MergedObject, Default)]
pub struct QueryRoot(MyBareQuestion);
//FetchContent

