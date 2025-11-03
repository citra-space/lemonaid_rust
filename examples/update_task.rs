use lemonaid::CitraClient;
use lemonaid::TaskStatus;
use lemonaid::TaskUpdateRequest;
use std::env;

#[tokio::main]
async fn main() {
    // Get API key from environment variable
    let api_key = env::var("CITRA_PAT")
        .expect("CITRA_PAT environment variable not set");

    // Create client
    let client = CitraClient::new(&api_key);

    // Get task ID from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example update_task <task_id>");
        std::process::exit(1);
    }
    let task_id = &args[1];

    // Test update_task
    println!("Updating task: {}", task_id);
    let update_request = TaskUpdateRequest {
        id: task_id.to_string(),
        status: TaskStatus::Canceled,
        priority: Some(5),
        scheduled_start: None,
        scheduled_stop: None,
    };
    match client.update_task(&update_request).await {
        Ok(updated_task) => {
            println!("\n✓ Success!");
            println!("{:#?}", updated_task);
        }
        Err(err) => {
            eprintln!("\n✗ Error updating task: {}", err);
        }
    }
}
