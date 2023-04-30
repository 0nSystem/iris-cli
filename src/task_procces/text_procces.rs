use std::collections::HashMap;

use crate::system_resources::{actions::get_file_to_string, model::config_file::ConfigFile};
use crate::petitions::client;

pub async fn config_text_command<'a>(
    text_translate_in_command: &'a Option<String>,
    text_file: &'a Option<std::path::PathBuf>,
    languaje: &'a str,
    config: &'a ConfigFile,
) -> Result<HashMap<String, String>, super::TaskError> {
    if let Some(text_command) = text_translate_in_command {
        return text_command_procces(text_command, languaje, config).await;
    } else if let Some(text_path_file) = text_file {
        let file_string = get_file_to_string(&text_path_file).map_err(|e| super::TaskError::ReadFile)?;
        return text_command_procces(&file_string, languaje, config).await;
    } else {
        return Err(super::TaskError::RequireField(
            "Error define text to translate in cli or a file".to_owned(),
        ));
    }
}

async fn text_command_procces<'a>(
    text: &'a str,
    languaje: &'a str,
    config: &'a ConfigFile,
) -> Result<HashMap<String, String>, super::TaskError> {
    let mut map_name_file_to_add_and_value_info_translate: HashMap<String,String> = HashMap::new();

    config.configurations.iter().enumerate().for_each(|(index,config)|{
        //TODO remove clone
        let name_file_to_add = config.name.clone().unwrap_or_else(||index.to_string());
        match client::build_client::build_client(config.authentication.as_ref()) {
            Ok(client) => {
                let request = client::build_request::build_request(&client::options_request_client::OptionClientRequest::from(config), text, languaje);
                client::send_request::send_request(&client, request);

            },
            Err(_) => todo!(),
        }

    });


    todo!()
}
