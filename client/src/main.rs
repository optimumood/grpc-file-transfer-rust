use client::client_main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    client_main().await?;
    Ok(())
}
