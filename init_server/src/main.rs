mod server;
#[tokio::main]
async fn main() {
    //initialize server at port 3000
    server::initialize_server().await;

}