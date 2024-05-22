use axum::{routing::{get,post}, Router};
use tower_http::cors::CorsLayer;
use axum::http::Method;


pub mod routes;
pub mod format;

pub async fn initialize_server(){

    //Initialize middleware CORS Layer
    let cors = CorsLayer::new() 
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(tower_http::cors::Any);

    let app = Router::new()
    .route("/", get(|| async { "root" }))
    .route("/mail_box/:query", get(routes::sender::send_question))
    .route("/mail_box", post(routes::receiver::check_answer))
	.layer(cors);

    //bind with port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}
