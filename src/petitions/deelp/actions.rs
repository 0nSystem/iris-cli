use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Response, StatusCode,
};

use crate::petitions::ErrorPetition;

use super::{
    constants::{NAME_API_KEY, URL_FREE_DEEPL},
    model::Message,
};

// TODO
pub fn build_client_to_deepl<'a>(api_key: &'a str) -> Client {
    let mut header = HeaderMap::new();
    header.append(
        AUTHORIZATION,
        format!("{} {}", NAME_API_KEY, api_key).parse().unwrap(),
    );

    Client::builder().default_headers(header).build().unwrap()
}

// TODO
pub async fn send_petition<'a, 'b>(
    client: &'a Client,
    params_request: &'b Message<'b>,
) -> Result<Response, ErrorPetition> {
    let params = [
        ("text", params_request.text),
        ("target_lang", params_request.cod_lang),
    ];

    let response = client
        .post(URL_FREE_DEEPL)
        .form(&params)
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK | StatusCode::ACCEPTED => Ok(response),
        _ => Err(ErrorPetition::StatusResponseNotValid(
            response.status().as_u16(),
        )),
    }
}