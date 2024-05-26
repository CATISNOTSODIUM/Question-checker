use async_graphql::{EmptySubscription, Schema};
use crate::server::routes::graphql::query::QueryRoot;
use crate::server::routes::graphql::mutation::MutationRoot;

pub fn build_my_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    
    Schema::build(
        QueryRoot::default(), 
        MutationRoot::default(), 
        EmptySubscription)
        //TODO: Add database here
        //.data(db)
        .finish()
}


