use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Response,
};

use crate::petitions::{ErrorPetition, validate_status_response};

use super::{
    constants::{NAME_API_KEY, URL_FREE_DEEPL},
    model::Message,
};

// TODO
pub fn build_client_to_deepl<'a>(api_key: &'a str) -> Client {
    let mut header = HeaderMap::new();
    header.append(
        AUTHORIZATION,
        format!("{} {}", NAME_API_KEY, api_key).parse().expect("Error building api key header value"),
    );

    Client::builder().default_headers(header).build().expect("Error building client to deepl")
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

    let result_response = client
        .post(URL_FREE_DEEPL)
        .form(&params)
        .send()
        .await;

    match result_response {
        Ok(response) => {
            validate_status_response(&response)?;
            return Ok(response);
        },
        Err(_) => Err(ErrorPetition::ErrorSendRequest(URL_FREE_DEEPL.to_string())),
    }
}

