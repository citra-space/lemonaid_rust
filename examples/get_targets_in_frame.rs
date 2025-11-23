use lemonaid::CitraClient;
use std::env;

#[tokio::main]
async fn main() {
    // Get API key from environment variable
    let api_key = env::var("CITRA_PAT")
        .expect("CITRA_PAT environment variable not set");

    // Create client
    let client = CitraClient::new(&api_key, true);

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Get targets in frame for groundstation at provided ra/dec and fov
    if args.len() < 5 {
        eprintln!("Usage: cargo run --example get_targets_in_frame <groundstation-id> <ra_deg> <dec_deg> <fov_deg>");
        std::process::exit(1);
    }
    let groundstation_id = &args[1];

    // get the groundstation to obtain its lat/lon/alt
    let groundstation = match client.get_groundstation(groundstation_id).await {
        Ok(gs) => gs,
        Err(e) => {
            eprintln!("\n✗ Error fetching groundstation: {}", e);
            std::process::exit(1);
        }
    };

    let ra_deg: f64 = args[2].parse().expect("Invalid number for ra_deg");
    let dec_deg: f64 = args[3].parse().expect("Invalid number for dec_deg");
    let fov_deg: f64 = args[4].parse().expect("Invalid number for fov_deg");
    let fov_request = lemonaid::FOVAccessRequest {
        time: chrono::Utc::now(),
        right_ascension_deg: ra_deg,
        declination_deg: dec_deg,
        field_of_view_deg: fov_deg,
        sensor_frame: lemonaid::SensorFrame::J2000,
        sensor_latitude_deg: groundstation.latitude,
        sensor_longitude_deg: groundstation.longitude,
        sensor_altitude_km: groundstation.altitude / 1000.0,
    };

    println!("Fetching targets in frame for groundstation: {} at RA: {}, Dec: {}, FOV: {}", 
        groundstation_id, ra_deg, dec_deg, fov_deg);
    match client.solve_fov_access(&fov_request).await {
        Ok(targets) => {
            println!("\n✓ Found {} target(s) in frame", targets.len());
            for target in targets {
                println!("  - Target ID: {}, Name: {:?}", target.satellite_id, target.satellite_name);
            }
        },
        Err(e) => {
            eprintln!("\n✗ Error fetching targets in frame: {}", e);
            std::process::exit(1);
        }
    }
}
