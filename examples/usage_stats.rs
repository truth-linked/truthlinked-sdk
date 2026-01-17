use truthlinked_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license_key = std::env::var("TRUTHLINKED_LICENSE_KEY")
        .expect("TRUTHLINKED_LICENSE_KEY not set");
    
    let client = Client::new("https://api.truthlinked.org", license_key)?;
    
    println!("Fetching usage statistics...\n");
    
    let usage = client.get_usage().await?;
    
    println!("📈 Usage Statistics");
    println!("==================");
    println!("Tier: {}", usage.tier);
    println!("Usage: {} / {} requests", usage.usage, usage.limit);
    println!("Percentage: {:.1}%", usage.percentage);
    println!("Days remaining: {}", usage.days_remaining);
    
    // Warning if approaching limit
    if usage.percentage > 80.0 {
        println!("\n⚠️  WARNING: Approaching rate limit!");
        println!("   Consider upgrading your tier.");
    }
    
    // Warning if license expiring soon
    if usage.days_remaining < 7 {
        println!("\n⚠️  WARNING: License expires in {} days!", usage.days_remaining);
        println!("   Contact support to renew.");
    }
    
    Ok(())
}
