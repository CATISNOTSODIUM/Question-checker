use axum::{response::IntoResponse, routing::get, Router};
use tower_http::cors::{CorsLayer,Any};
use self::routes::serve_files::read_markdown;
pub mod routes;

use routes::graphql::schema::build_my_schema;

// for graphql playground
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;

// Graph iql playground handler
async fn graphiql() -> impl IntoResponse {
    axum::response::Html(GraphiQLSource::build().endpoint("/api/graphql").finish())
}


// Initialize server
pub async fn initialize_server() -> Result<(), Box<dyn std::error::Error>> {

    //Initialize middleware CORS Layer
    let cors = CorsLayer::new() 
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_origin(Any);

    let my_schema = build_my_schema();

    
    let app = Router::new()
    .route("/", get(|| async { "root" }))
    //.route("/api/graph_ql", post_service(GraphQL::new(my_schema))) 
    // Graphql playground
    .route("/api/graphql", get(graphiql).post_service(GraphQL::new(my_schema)))
    // Serve markdown file (RestAPI)
    .route("/api/markdown", get(read_markdown))
    .layer(cors);

    //bind with port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}



