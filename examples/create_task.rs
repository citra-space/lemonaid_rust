use lemonaid::CreateTaskRequest;
use chrono::Utc;
use std::env;

#[tokio::main]
async fn main() {
    // Get API key from environment variable
    let api_key = env::var("CITRA_PAT")
        .expect("CITRA_PAT environment variable not set");

    // Get task ID from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run --example create_task <telescope_id>, <satellite_id>");
        std::process::exit(1);
    }
    let telescope_id = &args[1];
    let satellite_id = &args[2];
    let start_time = Utc::now();
    let stop_time = start_time + chrono::Duration::hours(1);

    // Test create_task with debug
    println!("Creating task for telescope: {}", telescope_id);
    let create_request = CreateTaskRequest {
        telescope_id: Some(telescope_id.to_string()),
        satellite_id: satellite_id.to_string(),
        antenna_id: None,
        task_start: start_time,
        task_stop: stop_time,
    };

    match lemonaid::CitraClient::new(&api_key).create_task(&create_request).await {
        Ok(created_task) => {
            println!("\n✓ Success!");
            println!("{:#?}", created_task);
        }
        Err(err) => {
            eprintln!("\n✗ Error creating task: {}", err);
        }
    }
}
