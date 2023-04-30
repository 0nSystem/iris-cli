use jsonpath_lib;
use reqwest::{Response, StatusCode};

#[derive(Debug)]
pub enum ErrorRequest {
    StatusResponseNotValid(u16),
    ErrorSendRequest(String),
    ParsePattern(String),
    CantGetBody,
}

pub fn handle_error_petition_log(error: &ErrorRequest) {
    match error {
        ErrorRequest::StatusResponseNotValid(number) => {
            log::error!("Failed response with status: {number}")
        }
        ErrorRequest::ErrorSendRequest(url) => log::error!("Failed send request in {url}"),
        ErrorRequest::ParsePattern(pattern) => log::error!("Error parse with pattern: {pattern}"),
        ErrorRequest::CantGetBody => log::error!("Cant get body value"),
    }
}

pub fn validate_status_response(response: &Response) -> Result<(), ErrorRequest> {
    match response.status() {
        StatusCode::OK | StatusCode::ACCEPTED => Ok(()),
        _ => Err(ErrorRequest::StatusResponseNotValid(
            response.status().as_u16(),
        )),
    }
}

pub fn get_values_json_by_pattern<'a>(
    json: &'a serde_json::Value,
    pattern_expresion: &'a str,
) -> Result<Vec<&'a serde_json::Value>, ErrorRequest> {
    match jsonpath_lib::select(json, pattern_expresion) {
        Ok(result_find_by_patter) => Ok(result_find_by_patter),
        Err(error) => {
            Err(ErrorRequest::ParsePattern(pattern_expresion.to_owned()))
            //TODO log
        }
    }
}

pub async fn create_and_management_response<'a>(
    client: &'a reqwest::Client,
    options_client_request: &'a super::client::options_request_client::OptionClientRequest<'a>,
    text: &'a str,
    language: &'a str,
    pattern_expresion: &'a str,
) -> Result<Vec<String>, ErrorRequest> {
    let request =
        super::client::build_request::build_request(&options_client_request, text, language);
    let reponse = super::client::send_request::send_request(&client, request).await?;
    let json_to_parse = &reponse
        .json()
        .await
        .map_err(|_| ErrorRequest::CantGetBody)?;

    let values_parse_json = get_values_json_by_pattern(json_to_parse, pattern_expresion)?;
    let values_filter_and_conversion_to_string: Vec<String> = values_parse_json
        .iter()
        .filter(|value| value.is_string())
        .map(|v| v.as_str().unwrap_or("").to_string())
        .collect();

    Ok(values_filter_and_conversion_to_string)
}
