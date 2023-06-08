use std::collections::HashMap;

use color_eyre::Report;
use jsonpath_lib::{replace_with, select, JsonPathError};
use reqwest::Client;
use serde_json::{json, Value};

use crate::petitions::{self, client, management_response};
use crate::{
    petitions::client::options_request_client,
    system_resources::{
        actions,
        model::config_file::{ApiParams, ConfigFile},
    },
    task_procces::TaskError,
};

pub async fn config_and_run_json_command<'a>(
    fields_to_translate: &'a Vec<String>, //pattern expression
    text_file: &'a Option<std::path::PathBuf>,
    languaje: &'a str,
    config: &'a ConfigFile,
) -> Result<HashMap<String, String>, super::TaskError> {
    let path_text_field_validated = text_file
        .as_ref()
        .ok_or_else(|| TaskError::RequireField("Input Text".to_owned()))?;
    let file_json_in_str =
        actions::get_file_to_string(path_text_field_validated).map_err(|_| TaskError::ReadFile)?;

    let file_json: Value = serde_json::from_str(&file_json_in_str).expect("deserilize");

    let mut map_add_alias_file_and_json_in_string = HashMap::new();

    for (i, conf) in config.configurations.iter().enumerate() {
        let name = conf.name.clone().unwrap_or_else(|| i.to_string()); //TODO remove clone with if's
        map_add_alias_file_and_json_in_string.insert(
            name,
            json_command_procces(conf, languaje, &file_json, fields_to_translate)
                .await
                .expect("json command"),
        );
    }

    Ok(map_add_alias_file_and_json_in_string)
}

//TODO async code multiples peticiones simulaneas
async fn json_command_procces<'a>(
    api_param: &'a ApiParams,
    languaje: &'a str,
    json_file: &'a Value,
    pattern_expresion: &'a Vec<String>,
) -> Result<String, Report> {
    let client = client::build_client::build_client(api_param.authentication.as_ref())?;

    let values_filtered_by_pattern_expresion =
        grouping_by_pattern_and_filter_value_json_string(pattern_expresion, json_file)?;

    let config_request = options_request_client::OptionClientRequest::from(api_param);

    let mut json_replace = json_file.clone();

    for entry_filtered_values in values_filtered_by_pattern_expresion {
        let translation_old_value_translate_value = translation_all_values(
            &client,
            &config_request,
            &entry_filtered_values.1,
            languaje,
            &api_param.get_value_json,
        )
        .await?;

        //replace
        json_replace = replace_with(json_replace, entry_filtered_values.0, &mut |f| {
            if let Some(text) = f.as_str() {
                return translation_old_value_translate_value
                    .get(&text.to_owned())
                    .map(|t| Value::String(t.clone()));
            }
            None
        })?;
    }

    Ok(serde_json::to_string_pretty(&json_replace)?)
}

async fn translation_all_values<'a>(
    client: &'a Client,
    config_request: &'a options_request_client::OptionClientRequest<'a>,
    text: &'a Vec<String>,
    languaje: &'a str,
    path_value_response: &'a str,
) -> Result<HashMap<&'a String, String>, Report> {
    let mut map_string_old_value_new_value = HashMap::new();

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

async fn translation<'a>(
    client: &'a Client,
    config_request: &'a options_request_client::OptionClientRequest<'a>,
    text: &'a str,
    languaje: &'a str,
    path_value_response: &'a str,
) -> Result<String, Report> {
    let response = management_response::create_and_management_response(
        client,
        config_request,
        text,
        languaje,
        path_value_response,
    )
    .await
    .expect("msg"); // TODO

    //TODO
    Ok(response.1.first().expect("msg").to_string())
}

fn grouping_by_pattern_and_filter_value_json_string<'a>(
    pattern_expresions: &'a Vec<String>,
    json_file: &'a Value,
) -> Result<HashMap<&'a String, Vec<String>>, Report> {
    let mut map = HashMap::new();

    for path_expression in pattern_expresions {
        let selected_values_filtered_by_str: Vec<String> = select(json_file, &path_expression)?
            .iter()
            .filter_map(|f| f.as_str().map(|p| p.to_string()))
            .collect();
        map.insert(path_expression, selected_values_filtered_by_str);
    }

    Ok(map)
}
