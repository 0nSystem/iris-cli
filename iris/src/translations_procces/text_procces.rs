use std::collections::HashMap;

use color_eyre::Result;

use crate::petitions::client::options_request_client;
use crate::petitions::config_request::MultiplesApiParams;
use crate::petitions::{client, translation};

pub async fn text_command(
    text: &str,
    languaje: &str,
    config: &MultiplesApiParams,
) -> Result<HashMap<String, String>> {
    let mut map_name_file_to_add_and_value_info_translate: HashMap<String, String> = HashMap::new();

    for (index, config) in config.configurations.iter().enumerate() {
        //TODO remove clone
        let name_file_to_add = config.name.clone().unwrap_or_else(|| index.to_string());
        let client = client::build_client::build_client(config.authentication.as_ref())?;
        let options_request = &options_request_client::OptionClientRequest::from(config);

        let translate = translation(
            &client,
            options_request,
            text,
            languaje,
            &config.get_value_json,
        )
        .await?;
        map_name_file_to_add_and_value_info_translate.insert(name_file_to_add, translate);
    }

    Ok(map_name_file_to_add_and_value_info_translate)
}
