/*!
 * Text command module
 */

use crate::request::client::options_request_client;
use crate::request::config_request::{ApiParams, MultiplesApiParams};
use crate::request::{client, translation};
use color_eyre::Result;
use std::collections::HashMap;

/// Text translation process with various api params configurations: [`MultiplesApiParams`], internally uses [`text_command`].
pub async fn text_command_with_multiples_api_params(
    text: &str,
    languaje: &str,
    config: &MultiplesApiParams,
) -> Result<HashMap<String, String>> {
    let mut map_name_file_to_add_and_value_info_translate: HashMap<String, String> = HashMap::new();

    for (index, config) in config.configurations.iter().enumerate() {
        let name_file_to_add = config.name.clone().unwrap_or_else(|| index.to_string());
        let text_translate = text_command(text, languaje, config).await?;
        map_name_file_to_add_and_value_info_translate.insert(name_file_to_add, text_translate);
    }

    Ok(map_name_file_to_add_and_value_info_translate)
}

/// Text translation process
pub async fn text_command(text: &str, languaje: &str, config: &ApiParams) -> Result<String> {
    let client = client::build_client(config.authentication.as_ref())?;
    let options_request = &options_request_client::OptionClientRequest::from(config);

    let translate = translation(
        &client,
        options_request,
        text,
        languaje,
        &config.get_value_json,
    )
    .await?;

    Ok(translate)
}
