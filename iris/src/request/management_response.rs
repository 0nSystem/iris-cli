/*!
 *  Manages the response to customer requests
 */
use super::client::{build_request, options_request_client::OptionClientRequest, send_request};
use color_eyre::Result;
use jsonpath_lib;

fn get_values_json_by_pattern<'a>(
    json: &'a serde_json::Value,
    pattern_expresion: &'a str,
) -> Result<Vec<&'a serde_json::Value>> {
    Ok(jsonpath_lib::select(json, pattern_expresion)?)
}
/// Manage the response to customer requests by collecting data,
/// returning a map with the key, the text to translate and a list of the translated value.
pub async fn create_and_management_response(
    client: &reqwest::Client,
    options_client_request: &OptionClientRequest,
    text: &str,
    language: &str,
    pattern_expresion: &str,
) -> Result<(String, Vec<String>)> {
    let request = build_request(options_client_request, text, language)?;
    let reponse = send_request(client, request).await?;
    let json_to_parse = &reponse.json().await?;

    let values_parse_json = get_values_json_by_pattern(json_to_parse, pattern_expresion)?;
    let values_filter_and_conversion_to_string: Vec<String> = values_parse_json
        .iter()
        .filter(|value| value.is_string())
        .map(|v| v.as_str().unwrap_or("").to_string())
        .collect();

    Ok((text.to_owned(), values_filter_and_conversion_to_string))
}
