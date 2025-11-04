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
        eprintln!("Usage: cargo run --example get_telescope <telescope-id>");
        std::process::exit(1);
    }
    let telescope_id = &args[1];

    // Test get_telescope
    println!("Fetching telescope: {}", telescope_id);
    match client.get_telescope(telescope_id).await {
        Ok(telescope) => {
            println!("\n✓ Success!");
            println!("{:#?}", telescope);
        }
        Err(e) => {
            eprintln!("\n✗ Error: {}", e);
            std::process::exit(1);
        }
    }

    // Test list_tasks_for_telescope with debug
    println!("\n\nFetching tasks for telescope: {}", telescope_id);
    match client.list_tasks_for_telescope(telescope_id).await {
        Ok(tasks) => {
            println!("\n✓ Found {} task(s) for telescope {}", tasks.len(), telescope_id);
            for task in tasks {
                println!("  - Task ID: {}, Status: {:?}", task.id, task.status);
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error fetching tasks for telescope: {}", e);
        }
    }

    // Test get_telescope_tasks_by_status
    let status_filter = vec![TaskStatus::Pending];
    println!("\n\nFetching tasks for telescope: {} with status: {:?}", telescope_id, status_filter);
    match client.get_telescope_tasks_by_status(telescope_id, status_filter).await {
        Ok(tasks) => {
            for task in tasks {
                println!("  - Task ID: {}, Status: {:?}", task.id, task.status);
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error fetching tasks for telescope by status: {}", e);
        }
    }

    // Test list_telescopes
    println!("\n\nFetching all telescopes...");
    match client.list_telescopes().await {
        Ok(telescopes) => {
            println!("\n✓ Found {} telescope(s)", telescopes.len());
            for telescope in telescopes {
                println!("  - {} ({})", telescope.name, telescope.id);
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error listing telescopes: {}", e);
        }
    }
}
