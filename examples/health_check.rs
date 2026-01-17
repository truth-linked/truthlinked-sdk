use truthlinked_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get license key from environment
    let license_key = std::env::var("TRUTHLINKED_LICENSE_KEY")
        .expect("TRUTHLINKED_LICENSE_KEY environment variable not set");
    
    // Create client
    let client = Client::new("https://api.truthlinked.org", license_key)?;
    
    // Check health
    println!("Checking server health...");
    let health = client.health().await?;
    println!("✅ Server status: {}", health.status);
    println!("   Version: {}", health.version);
    
    Ok(())
}
