use color_eyre::{Report, Result};

use self::client::options_request_client;

pub mod client;
pub mod constants;
pub mod management_response;
//TODO cambiar accesibilidad modulos

pub async fn translation_all_values<'a>(
    client: &'a reqwest::Client,
    config_request: &'a client::options_request_client::OptionClientRequest<'a>,
    text: &'a Vec<String>,
    languaje: &'a str,
    path_value_response: &'a str,
) -> Result<std::collections::HashMap<&'a String, String>, color_eyre::Report> {
    let mut map_string_old_value_new_value = std::collections::HashMap::new();

    for text_to_translate in text {
        let translation = translation(
            client,
            config_request,
            text_to_translate,
            languaje,
            path_value_response,
        )
        .await?;
        map_string_old_value_new_value.insert(text_to_translate, translation);
    }

    Ok(map_string_old_value_new_value)
}

pub async fn translation<'a>(
    client: &'a reqwest::Client,
    config_request: &'a options_request_client::OptionClientRequest<'a>,
    text: &'a str,
    languaje: &'a str,
    path_value_response: &'a str,
) -> Result<String> {
    let response = management_response::create_and_management_response(
        client,
        config_request,
        text,
        languaje,
        path_value_response,
    )
    .await?; // TODO

    println!("reponse: {:?}", response);
    //TODO
    Ok(response
        .1
        .first()
        .ok_or_else(|| Report::msg("Error empty body response"))?
        .to_string())
}
