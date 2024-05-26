mod server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //initialize server at port 3000
    server::initialize_server().await?;
    
    println!("Server has been initialized at port 3000.");
    Ok(())
}