use truthlinked_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license_key = std::env::var("TRUTHLINKED_LICENSE_KEY")
        .expect("TRUTHLINKED_LICENSE_KEY not set");
    
    let client = Client::new("https://api.truthlinked.org", license_key)?;
    
    println!("Fetching compliance reports...\n");
    
    // SOX compliance
    println!("📊 SOX Compliance Report");
    println!("========================");
    let sox = client.get_sox_report().await?;
    println!("Period: {}", sox.period);
    println!("Total events: {}", sox.total_events);
    println!("Audit trail complete: {}", if sox.audit_trail_complete { "✅" } else { "❌" });
    println!("No gaps: {}", if sox.no_gaps { "✅" } else { "❌" });
    
    println!("\n📊 PCI-DSS Compliance Report");
    println!("============================");
    let pci = client.get_pci_report().await?;
    println!("Period: {}", pci.period);
    println!("Access controls enforced: {}", if pci.access_controls_enforced { "✅" } else { "❌" });
    println!("Encryption verified: {}", if pci.encryption_verified { "✅" } else { "❌" });
    println!("Audit complete: {}", if pci.audit_complete { "✅" } else { "❌" });
    
    Ok(())
}
