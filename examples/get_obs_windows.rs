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

    // Get observation windows for groundstation
    if args.len() < 3 {
        eprintln!("Usage: cargo run --example get_obs_windows <groundstation-id> <minutes_from_now>");
        std::process::exit(1);
    }
    let groundstation_id = &args[1];
    let minutes_from_now: i64 = args[2].parse().expect("Invalid number for minutes_from_now");
    let start_time = chrono::Utc::now();
    let end_time = start_time + chrono::Duration::minutes(minutes_from_now);
    let access_request = lemonaid::SatelliteAccessToGroundstationRequest {
        groundstation_id: groundstation_id.to_string(),
        start: start_time,
        end: end_time,
        min_elevation_deg: 10.0,
        min_duration_minutes: 1.0,
        min_frequency_mhz: None,
        max_frequency_mhz: None,
    };

    println!("Fetching observation windows for groundstation: {} from {} to {}", 
        groundstation_id, start_time, end_time);
    match client.solve_access_for_groundstation(&access_request).await {
        Ok(windows) => {
            println!("\n✓ Found {} observation window(s) for groundstation {}", windows.len(), groundstation_id);
            for window in windows {
                println!("  - Satellite: {} ({})", window.satellite_name.unwrap_or("Unknown".to_string()), window.satellite_id);
                println!("    Start: {} (Az: {:.2}°, El: {:.2}°)", window.start.time, window.start.azimuth_deg, window.start.elevation_deg);
                println!("    End:   {} (Az: {:.2}°, El: {:.2}°)", window.end.time, window.end.azimuth_deg, window.end.elevation_deg);
                println!("    Duration: {:.2} minutes\n", window.duration_minutes);
            }
        }
        Err(e) => {
            eprintln!("\n✗ Error: {}", e);
            std::process::exit(1);
        }
    }
}
