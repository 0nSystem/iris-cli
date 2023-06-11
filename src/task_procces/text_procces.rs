use std::collections::HashMap;

use color_eyre::{Report, Result};

use crate::petitions::client::options_request_client;
use crate::petitions::{client, management_response};
use crate::system_resources::{actions::get_file_to_string, model::config_file::ConfigFile};

pub async fn config_and_run_text_command(
    text_translate_in_command: &Option<String>,
    text_file: &Option<std::path::PathBuf>,
    languaje: &str,
    config: &ConfigFile,
) -> Result<HashMap<String, String>> {
    if let Some(text_command) = text_translate_in_command {
        text_command_procces(text_command, languaje, config).await
    } else if let Some(text_path_file) = text_file {
        let file_string = get_file_to_string(text_path_file)?;
        text_command_procces(&file_string, languaje, config).await
    } else {
        Err(Report::msg("Require text to translate"))
    }
}

//todo refactor
async fn text_command_procces(
    text: &str,
    languaje: &str,
    config: &ConfigFile,
) -> Result<HashMap<String, String>> {
    let mut map_name_file_to_add_and_value_info_translate: HashMap<String, String> = HashMap::new();

    //TODO change to pararel request and logs
    for (index, config) in config.configurations.iter().enumerate() {
        //TODO remove clone
        let name_file_to_add = config.name.clone().unwrap_or_else(|| index.to_string());
        match client::build_client::build_client(config.authentication.as_ref()) {
            Ok(client) => {
                let options_request = &options_request_client::OptionClientRequest::from(config);
                match management_response::create_and_management_response(
                    &client,
                    options_request,
                    text,
                    languaje,
                    &config.get_value_json,
                )
                .await
                {
                    Ok(value_request) => {
                        if let Some(value_verification) = value_request.1.first() {
                            map_name_file_to_add_and_value_info_translate
                                .insert(name_file_to_add, value_verification.clone());
                            //TODO clone
                        };
                    }
                    Err(_) => todo!(),
                }
            }
            Err(_) => todo!(),
        };
    }

    Ok(map_name_file_to_add_and_value_info_translate)
}
