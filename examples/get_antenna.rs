use lemonaid::CitraClient;
use lemonaid::TaskStatus;
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

    // Test list_tasks_for_antenna
    println!("\n\nFetching tasks for antenna: {}", antenna_id);
    match client.list_tasks_for_antenna(antenna_id).await {
        Ok(tasks) => {
            println!("\n✓ Found {} task(s) for antenna {}", tasks.len(), antenna_id);
            for task in tasks {
                println!("  - Task ID: {}, Status: {:?}", task.id, task.status);
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error fetching tasks for antenna: {}", e);
        }
    }

    // Test get_antenna_tasks_by_status
    let status_filter = vec![TaskStatus::Pending];
    println!("\n\nFetching tasks for antenna: {} with status: {:?}", antenna_id, status_filter);
    match client.get_antenna_tasks_by_status(antenna_id, status_filter).await {
        Ok(tasks) => {
            for task in tasks {
                println!("  - Task ID: {}, Status: {:?}", task.id, task.status);
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error fetching tasks for antenna by status: {}", e);
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
