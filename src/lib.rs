//! HTTP client for the [Citra Space](https://citra.space) API.
//!
//! All request/response types and the [`Client`] are generated at build time
//! from the OpenAPI specification vendored at `openapi/citra.json`. To refresh
//! the vendored spec, run `scripts/update-openapi.sh`.
//!
//! Use [`CitraClient::new`] to construct a [`Client`] pre-configured with the
//! appropriate base URL (dev or prod) and a bearer-token `Authorization`
//! header on every outgoing request.
//!
//! ```no_run
//! use lemonaid::CitraClient;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CitraClient::new(&std::env::var("CITRA_PAT")?, /* dev */ true);
//! // Call generated methods on `client` — see the [`Client`] docs for the
//! // full list of operations.
//! # Ok(()) }
//! ```
//!
//! Endpoints not covered by the OpenAPI specification (for example direct
//! S3 uploads of capture artifacts) are implemented in handwritten modules
//! alongside the generated code.

#[allow(
    clippy::all,
    clippy::pedantic,
    unused_imports,
    unused_qualifications,
    dead_code,
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/citra_api.rs"));
}

pub use generated::*;

const PROD_BASE_URL: &str = "https://api.citra.space";
const DEV_BASE_URL: &str = "https://dev.api.citra.space";

/// Factory for building a generated [`Client`] pointed at the dev or prod
/// Citra Space API with a personal access token attached as a bearer header.
pub struct CitraClient;

impl CitraClient {
    /// Build a [`Client`] with `Authorization: Bearer <api_key>` set as a
    /// default header. When `dev` is true the client targets
    /// `https://dev.api.citra.space`; otherwise it targets
    /// `https://api.citra.space`.
    ///
    /// # Panics
    ///
    /// Panics if `api_key` contains characters that are invalid in an HTTP
    /// header value, or if the underlying [`reqwest::Client`] fails to build.
    pub fn new(api_key: &str, dev: bool) -> Client {
        let base_url = if dev { DEV_BASE_URL } else { PROD_BASE_URL };
        let mut headers = reqwest::header::HeaderMap::new();
        let mut auth = reqwest::header::HeaderValue::from_str(&format!("Bearer {api_key}"))
            .expect("API key contained characters invalid for an HTTP header");
        auth.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, auth);
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build reqwest client");
        Client::new_with_client(base_url, http)
    }
}
