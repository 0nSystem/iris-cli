/*!
* This module is responsible for the creation of clients and their own management,
* to provide a use as simple as possible and return a result with the parsed response,
* emphasizing the greatest possible automation, as it relies on the internal functions and structures of the module.
*/

use self::client::options_request_client;
use color_eyre::{eyre::Context, Report, Result};

pub mod client;
pub mod config_request;
pub mod constants;
pub mod management_response;

/// Allows a translation of a set of values, managed asynchronously for speed,
/// possible using internally [`translation`]
pub async fn translation_all_values<'a>(
    client: &reqwest::Client,
    config_request: &client::options_request_client::OptionClientRequest,
    text: &'a [String],
    languaje: &'a str,
    path_value_response: &'a str,
) -> Result<std::collections::HashMap<&'a String, String>, color_eyre::Report> {
    let mut map_string_old_value_new_value = std::collections::HashMap::new();

    let mut task_futures = Vec::new();

    for text_to_translate_i in text {
        let client = client.clone();
        let config_request = config_request.clone();
        let languaje = languaje.to_owned();
        let path_value_response = path_value_response.to_owned();
        let text_to_translate = text_to_translate_i.to_owned();

        let future = tokio::spawn(async move {
            translation(
                &client,
                &config_request,
                &text_to_translate,
                &languaje,
                &path_value_response,
            )
            .await
        });
        task_futures.push((text_to_translate_i, future));
    }

    for task in task_futures {
        map_string_old_value_new_value.insert(
            task.0,
            task.1
                .await
                .with_context(|| format!("Error wait finish task translate text {}", task.0))??,
        );
    }

    Ok(map_string_old_value_new_value)
}

/// Allows to perform a translation of a value, managed asynchronously,
/// using a client by creating a query to the configured server and managing the response.
/// Internally using [`management_response::create_and_management_response`]
pub async fn translation(
    client: &reqwest::Client,
    config_request: &options_request_client::OptionClientRequest,
    text: &str,
    languaje: &str,
    path_value_response: &str,
) -> Result<String> {
    let response = management_response::create_and_management_response(
        client,
        config_request,
        text,
        languaje,
        path_value_response,
    )
    .await?;

    Ok(response
        .1
        .first()
        .ok_or_else(|| Report::msg("Error empty body response"))?
        .to_string())
}
