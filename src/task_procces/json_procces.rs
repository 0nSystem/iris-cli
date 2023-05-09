use std::collections::HashMap;

use color_eyre::Report;
use jsonpath_lib::{select, JsonPathError, selector, SelectorMut, Selector};
use serde_json::{json, Value};
use std::hash::{Hash, Hasher};

use crate::{system_resources::{model::config_file::{ConfigFile, ApiParams}, actions}, task_procces::TaskError, petitions::client::options_request_client};
use crate::petitions::{client,management_response};


pub async fn config_and_run_json_command<'a>(
    fields_to_translate: &'a Vec<String>, //pattern expression 
    text_file: &'a Option<std::path::PathBuf>,
    languaje: &'a str,
    config: &'a ConfigFile,
) -> Result<HashMap<String, String>, super::TaskError> {

    let path_text_field_validated = text_file.as_ref().ok_or_else(||TaskError::RequireField("Input Text".to_owned()))?;
    let file_json_in_str = actions::get_file_to_string(&path_text_field_validated).map_err(|_|TaskError::ReadFile)?;
    
    let file_json = json!(file_json_in_str); 

    let mut map_add_alias_file_and_json_in_string = HashMap::new();

    for (i,conf) in config.configurations.iter().enumerate() {
        let name = conf.name.clone().unwrap_or_else(||i.to_string()); //TODO remove clone with if's
        match json_command_procces(conf, languaje,&file_json, fields_to_translate).await {
            Ok(json_string) => map_add_alias_file_and_json_in_string.insert(name, json_string),
            Err(_) => todo!(),
        };

    }

    Ok(map_add_alias_file_and_json_in_string)
    
}
//TODO async code multiples peticiones simulaneas
async fn json_command_procces<'a>(api_param: &'a ApiParams,languaje: &'a str,json_file: &'a Value, pattern_expresion: &'a Vec<String>) -> Result<String, Report>{
    let client = client::build_client::build_client(api_param.authentication.as_ref())?;
    let mut selector_json_file = jsonpath_lib::selector(json_file);

    let mut map_file_json_path_expresion_with_string_values: HashMap<&String,Vec<&Value>> = HashMap::new(); 

    for path_expresion in pattern_expresion{
        map_file_json_path_expresion_with_string_values.insert(path_expresion, selector_json_file(path_expresion)?);
    }

    let mut tuples_value_json_translate_value: Vec<(&Value, String)> = Vec::new();

    for entry_path_expression_values in map_file_json_path_expresion_with_string_values {
        for value_json in entry_path_expression_values.1 {
            let parse_str_value_json = value_json.as_str().expect("error parse");
            let options_request = &options_request_client::OptionClientRequest::from(api_param);
            let response = management_response::create_and_management_response(&client, options_request, parse_str_value_json, languaje, &api_param.get_value_json).await.map_err(|_e| super::TaskError::Request);

            match response {
                Ok(value_in_body) => {
                    let first_value_in_body = value_in_body.first().expect("");
                    tuples_value_json_translate_value.push((value_json, first_value_in_body.clone()));
                },
                Err(_) => todo!(),
            }
        }
    }

    for entry_value_json_and_value_translate in tuples_value_json_translate_value {
        entry_value_json_and_value_translate.0.as_str().replace(entry_value_json_and_value_translate.1.as_str());
    }

    let mut json_string_result = json_file.to_string().replace(r"\n", "\n").replace(r#"\"#, "");
    json_string_result.remove(0);
    json_string_result.remove(json_string_result.len()-1);
    Ok(json_string_result)
}


