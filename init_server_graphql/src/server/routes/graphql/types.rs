// General types


use async_graphql::InputObject;

#[derive(InputObject)]
pub struct User {
    // More probs can be added for authentication.
    pub display_name: String,
}

