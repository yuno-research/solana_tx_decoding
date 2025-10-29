use reqwest::Client;
use serde_json::Value;

/**
Not all token metadata is stored on the blockchain. The blockchain stores a URI which is a link to
an API that returns a json object of additonal metadata. From this object, we look to extract the
description, twitter, and website, if these fields exist. If they do not, then we can return empty
strings for them.
*/
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