use std::collections::HashMap;

use color_eyre::Report;

use crate::petitions::constants;
use crate::system_resources::model;

pub fn create_default_template() -> Result<HashMap<String, String>, Report> {
    let mut map_name_to_add_file_and_info_template = HashMap::new();

    let config_json = model::config_file::ConfigFile {
        configurations: vec![model::config_file::ApiParams {
            name: Some("deepl".to_owned()),
            method_request: model::config_file::MethodRequest::Post,
            url: "https://api-free.deepl.com/v2/translate".to_owned(),
            authentication: Some("<authentication>".to_owned()),
            params_request: vec![model::config_file::ParamRequest::InUri(format!(
                "text={}&target_lang={}",
                constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE,
                constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE
            ))],
            get_value_json: "$..text".to_owned(),
        }],
    };

    let serialize_template = serde_json::ser::to_string_pretty(&config_json)?;

    map_name_to_add_file_and_info_template.insert("default".to_owned(), serialize_template);

    Ok(map_name_to_add_file_and_info_template)
}
