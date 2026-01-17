use truthlinked_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license_key = std::env::var("TRUTHLINKED_LICENSE_KEY")
        .expect("TRUTHLINKED_LICENSE_KEY not set");
    
    let client = Client::new("https://api.truthlinked.org", license_key)?;
    
    println!("Fetching shadow decisions (breach detections)...\n");
    
    let decisions = client.get_shadow_decisions().await?;
    
    if decisions.is_empty() {
        println!("✅ No breaches detected (good news!)");
    } else {
        println!("⚠️  Found {} divergences:\n", decisions.len());
        
        let breaches: Vec<_> = decisions.iter()
            .filter(|d| d.breach_prevented)
            .collect();
        
        println!("🛡️  Breaches prevented: {}", breaches.len());
        
        for decision in breaches {
            println!("\n  Divergence ID: {}", decision.divergence_id);
            println!("  IAM allowed: {}", decision.iam_allowed);
            println!("  AF would allow: {}", decision.af_would_allow);
            println!("  ⚠️  BREACH PREVENTED");
        }
    }
    
    Ok(())
}
