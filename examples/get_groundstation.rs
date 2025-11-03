use lemonaid::CitraClient;
use std::env;

#[tokio::main]
async fn main() {
    // Get API key from environment variable
    let api_key = env::var("CITRA_PAT")
        .expect("CITRA_PAT environment variable not set");

    // Create client
    let client = CitraClient::new(&api_key);

    // Get groundstation ID from command line argument if provided
    let args: Vec<String> = env::args().collect();
    
    if args.len() >= 2 {
        let groundstation_id = &args[1];
        
        // Test get_groundstation
        println!("Fetching groundstation: {}", groundstation_id);
        match client.get_groundstation(groundstation_id).await {
            Ok(groundstation) => {
                println!("\n✓ Success!");
                println!("{:#?}", groundstation);
            }
            Err(e) => {
                eprintln!("\n✗ Error: {}", e);
                std::process::exit(1);
            }
        }
        
        println!("\n");
    }

    // Test list_groundstations
    println!("Fetching all groundstations...");
    match client.list_groundstations().await {
        Ok(groundstations) => {
            println!("\n✓ Found {} groundstation(s)", groundstations.len());
            for groundstation in groundstations {
                println!("  - {} ({}) at {}, {}", 
                    groundstation.name, 
                    groundstation.id,
                    groundstation.latitude,
                    groundstation.longitude
                );
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error: {}", e);
        }
    }
}
