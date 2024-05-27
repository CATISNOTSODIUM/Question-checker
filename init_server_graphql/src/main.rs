mod server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Server has been initialized at http://localhost:3000/.");
    println!("GRAPHQL: http://localhost:3000/api/graphql.");
    println!("MARKDOWN: http://localhost:3000/api/markdown.");
    //initialize server at port 3000
    server::initialize_server().await?;
    Ok(())
}