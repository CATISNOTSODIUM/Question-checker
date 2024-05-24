use serde::{Deserialize, Serialize};
use axum::{http::request::Parts, routing::{get, post}, Router};
use tower_http::cors::{CorsLayer,Any};
use std::sync::Arc;
use specta::Type;

use crate::server::format::response_type::MyBareResponse;

use self::routes::serve_files::read_markdown;

pub mod routes;
pub mod format;

pub struct Ctx {}

pub async fn initialize_server() -> Result<(), Box<dyn std::error::Error>> {

    //Initialize middleware CORS Layer
    let cors = CorsLayer::new() 
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_origin(Any);

    let my_rspc_router = rspc_router();
    
    let app = Router::new()
    .route("/", get(|| async { "root" }))
    .nest(
        "/rspc",
        rspc_axum::endpoint(
            my_rspc_router,
            |parts: Parts| {
                println!("Client requested operation '{}'", parts.uri.path());
                Ctx {}
            }
        ),
    )
    .route("/rspc/serve_files", post(read_markdown)) //use REST API for serving files
    .layer(cors);

    //bind with port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}


//All rspc routes
pub fn rspc_router() -> Arc<rspc::Router<Ctx>> {

    let router =
        rspc::Router::<Ctx>::new()
            .query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")))
            //no context (ctx) is currently added.
            .query("fetch_question", |t| {
                t(|_ctx, res : FetchQuestion | async move {
                    (match routes::sender::send_question(&res.file_path,&res.question_id).await{
                        //valid response
                        Ok(res) => Ok(res),
                        //Error: return as rspc error
                        Err(e) => Err(rspc::Error::new(rspc::ErrorCode::InternalServerError,
                            format!("{:#}",e).into()) 
                        )
                    }) as Result<String, rspc::Error>
                })
            })
            //no context (ctx) is currently added.
            .mutation("check_answer", |t| { 
                t(|_ctx , res: CheckAnswer | async move {
                    (match routes::receiver::check_answer(&res.file_path, res.payload).await{
                        //valid response
                        Ok(r) => Ok(r),
                        //Error: return as rspc error
                        Err(e) => Err(rspc::Error::new(rspc::ErrorCode::InternalServerError,
                            format!("{:#}",e).into()) 
                        )
                    }) as Result<String, rspc::Error>

                })
            })
            .build()
            .arced(); // This function is a shortcut to wrap the router in an `Arc`.
    return router;
}

#[derive(Type, Deserialize, Serialize)]
pub struct FetchQuestion {
    file_path: String,
    question_id: String,
}

#[derive(Type, Deserialize, Serialize)]
pub struct CheckAnswer {
    file_path: String,
    payload: MyBareResponse,
}