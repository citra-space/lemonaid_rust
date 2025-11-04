use lemonaid::CitraClient;
use std::env;

#[tokio::main]
async fn main() {
    // Get API key from environment variable
    let api_key = env::var("CITRA_PAT")
        .expect("CITRA_PAT environment variable not set");

    // Create client
    let client = CitraClient::new(&api_key);

    // Get telescope ID from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example get_antenna <antenna-id>");
        std::process::exit(1);
    }
    let antenna_id = &args[1];

    // Test get_antenna
    println!("Fetching antenna: {}", antenna_id);
    match client.get_antenna(antenna_id).await {
        Ok(antenna) => {
            println!("\n✓ Success!");
            println!("{:#?}", antenna);
        }
        Err(e) => {
            eprintln!("\n✗ Error: {}", e);
            std::process::exit(1);
        }
    }

    // Test list_antennas
    println!("\n\nFetching all antennas...");
    match client.list_antennas().await {
        Ok(antennas) => {
            println!("\n✓ Found {} antenna(s)", antennas.len());
            for antenna in antennas {
                println!("  - {} ({})", antenna.name, antenna.id);
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error listing antennas: {}", e);
        }
    }
}
