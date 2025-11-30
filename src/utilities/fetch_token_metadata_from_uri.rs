use reqwest::Client;
use serde_json::Value;

/// Fetch additional token metadata from off-chain URI. Token metadata URIs point to JSON APIs that
/// return additional metadata. This function extracts description, twitter, and website fields
/// from the JSON response, returning empty strings if fields are missing or not strings.
pub async fn fetch_token_metadata_from_uri(
  client: &Client,
  uri: &str,
) -> (String, String, String) {
  // Make the async GET request and panic if it fails
  let response = client
    .get(uri)
    .send()
    .await
    .expect("Failed to make GET request");

  // Parse the response as JSON and panic if it fails
  let data: Value = response.json().await.expect("Failed to parse JSON response");

  // Extract fields, using empty string if field doesn't exist or isn't a string
  let description = data
    .get("description")
    .and_then(|v| v.as_str())
    .unwrap_or("")
    .to_string();

  let twitter = data
    .get("twitter")
    .and_then(|v| v.as_str())
    .unwrap_or("")
    .to_string();

  let website = data
    .get("website")
    .and_then(|v| v.as_str())
    .unwrap_or("")
    .to_string();

  (description, twitter, website)
}