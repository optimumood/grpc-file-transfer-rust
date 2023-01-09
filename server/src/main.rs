use server::server_main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server_main().await?;
    Ok(())
}
