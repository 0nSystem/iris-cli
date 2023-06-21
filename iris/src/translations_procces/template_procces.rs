/*!
 * Allows you to create default templates, to make quick use of the application.
 */

use color_eyre::Result;
use std::collections::HashMap;

use crate::request::{
    config_request::{ApiParams, MethodRequest, MultiplesApiParams, ParamRequest},
    constants,
};

/// Create a default template, in this case to use a deepl api configuration.
pub fn create_default_template() -> Result<HashMap<String, String>> {
    let mut map_name_to_add_file_and_info_template = HashMap::new();

    let config_json = MultiplesApiParams {
        configurations: vec![ApiParams {
            name: Some("deepl".to_owned()),
            method_request: MethodRequest::Post,
            url: "https://api-free.deepl.com/v2/translate".to_owned(),
            authentication: Some("<authentication>".to_owned()),
            params_request: vec![ParamRequest::InUri(format!(
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
