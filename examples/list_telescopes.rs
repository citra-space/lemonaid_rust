//! List the telescopes registered to your Citra Space account.
//!
//! Run with:
//!
//! ```text
//! CITRA_PAT=... cargo run --example list_telescopes
//! ```

use lemonaid::CitraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("CITRA_PAT").expect("CITRA_PAT environment variable not set");
    let dev = std::env::var("CITRA_PROD").is_err();

    let client = CitraClient::new(&api_key, dev);

    let response = client.list_telescopes_telescopes_get(None).await?;
    let telescopes = response.into_inner();

    println!("Found {} telescope(s)", telescopes.len());
    for t in telescopes {
        println!("  {:?} (id={})", t.name, t.id);
    }

    Ok(())
}
