use std::collections::HashMap;

use color_eyre::Result;
use jsonpath_lib::{replace_with, select};
use serde_json::Value;

use crate::petitions::client::options_request_client;
use crate::petitions::config_request::{ApiParams, MultiplesApiParams};
use crate::petitions::{client, translation_all_values};

pub async fn json_command_with_multiples_api_params<'a>(
    fields_to_translate: &'a Vec<String>, //pattern expression
    text: &'a str,
    languaje: &'a str,
    config: &'a MultiplesApiParams,
) -> Result<HashMap<String, String>> {
    let file_json: Value = serde_json::from_str(text)?;

    let mut map_add_alias_file_and_json_in_string = HashMap::new();

    for (i, conf) in config.configurations.iter().enumerate() {
        let name = conf.name.clone().unwrap_or_else(|| i.to_string());
        map_add_alias_file_and_json_in_string.insert(
            name,
            json_command(conf, languaje, &file_json, fields_to_translate).await?,
        );
    }

    Ok(map_add_alias_file_and_json_in_string)
}

pub async fn json_command<'a>(
    api_param: &'a ApiParams,
    languaje: &'a str,
    json_file: &'a Value,
    pattern_expresion: &'a Vec<String>,
) -> Result<String> {
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

fn grouping_by_pattern_and_filter_value_json_string<'a>(
    pattern_expresions: &'a Vec<String>,
    json_file: &'a Value,
) -> Result<HashMap<&'a String, Vec<String>>> {
    let mut map = HashMap::new();

    for path_expression in pattern_expresions {
        let selected_values_filtered_by_str: Vec<String> = select(json_file, path_expression)?
            .iter()
            .filter_map(|f| f.as_str().map(|p| p.to_string()))
            .collect();
        map.insert(path_expression, selected_values_filtered_by_str);
    }

    Ok(map)
}
