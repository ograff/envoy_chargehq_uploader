use reqwest::blocking::{Client, ClientBuilder};
use serde::Serialize;
use std::env;

const CHARGEHQ_API_URL: &str = "https://api.chargehq.net/api/public/push-solar-data";

#[derive(Debug, Serialize)]
struct ChargeHqPayload {
    #[serde(rename = "apiKey")]
    api_key: String,
    #[serde(rename = "siteMeters")]
    site_meters: SiteMeter,
}

#[derive(Debug, Serialize)]
struct SiteMeter {
    production_kw: f32,
    net_import_kw: f32,
    consumption_kw: f32,
}
fn main() {
    // Retrieve the environment variables
    let source_host = env::var("SOURCE_HOST").expect("SOURCE_HOST not set");
    let charge_hq_api_key = env::var("CHARGE_HQ_API_KEY").expect("CHARGE_HQ_API_KEY not set");
    let envoy_jwt = env::var("ENVOY_JWT").expect("ENVOY_JWT not set");

    // Create a custom client with insecure configuration
    let client: Client = create_insecure_client();

    // Build the URL
    let url = format!("https://{}/production.json?details=1", source_host);

    // Send the request
    let response = client
        .get(&url)
        .bearer_auth(envoy_jwt)
        .send()
        .expect("Failed to send request");

    // Check the response status
    if response.status().is_success() {
        // Parse the JSON response
        let data: enphase::envoy::EnergyStats = response.json().expect("Failed to parse JSON");

        // Do something with the data...
        let consumption = (data.consumption.total.watts_now / 10.0).round() / 100.0;
        let production = (data.production.summary.watts_now / 10.0).round() / 100.0;
        let grid = ((consumption - production) * 100.0).round() / 100.0;

        let charge_hq_payload = ChargeHqPayload {
            api_key: charge_hq_api_key,
            site_meters: SiteMeter {
                production_kw: production,
                net_import_kw: grid,
                consumption_kw: consumption,
            },
        };
        let client = Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT, "text/plain".parse().unwrap());

        let json_printable = serde_json::to_string_pretty(&charge_hq_payload).unwrap();
        println!("{}", json_printable);
        let resp = client
            .post(CHARGEHQ_API_URL)
            .headers(headers)
            .body(serde_json::to_string(&charge_hq_payload).expect("Failed to serialize JSON"))
            .send()
            .expect("Failed to send request");
        println!("{:?}", resp);
        println!("{:?}", resp.text());
    } else {
        println!("Request failed with status code: {}", response.status());
    }
}

fn create_insecure_client() -> Client {
    // Create a custom client with insecure configuration
    let client_builder = ClientBuilder::new();
    let client = client_builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    client
}
