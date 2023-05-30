use std::collections::HashMap;

use color_eyre::Report;
use jsonpath_lib::{select, JsonPathError, selector, SelectorMut, Selector, replace_with};
use reqwest::Client;
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

    for (pattern, values) in map_file_json_path_expresion_with_string_values {
        let values_json_filter_as_string: Vec<&str> = values.iter().filter_map(|f|f.as_str()).collect();
        
        let json_value_string_from_translate = translate_fields(
            &client,
            &values_json_filter_as_string, 
            &pattern,
            &options_request_client::OptionClientRequest::from(api_param),
            languaje
        ).await.expect(""); //TODO

        replace_elements(json_file.clone(), pattern, &json_value_string_from_translate).expect("");
    }
    



    let mut json_string_result = json_file.to_string().replace(r"\n", "\n").replace(r#"\"#, "");
    json_string_result.remove(0);
    json_string_result.remove(json_string_result.len()-1);
    Ok(json_string_result)
}


fn replace_elements<'a> (json: Value,pattern: &'a str, translations: &'a HashMap<String,String>) -> Result<Value, JsonPathError>{
    replace_with(json, &pattern, &mut |f| {
        if let Value::String(n) = f {
            return translations.get(&n).map(|a|json!(a))
        }else{
            return Some(f) //TODO
        }
    })
}

async fn translate_fields<'a>(
    client:&'a Client,
    value_fields: &'a Vec<&'a str>,
    pattern: &'a str,
    options_request: &'a options_request_client::OptionClientRequest<'a>,
    languaje: &'a str
) -> Result<HashMap<String,String>,color_eyre::Report> {
    let mut json_value_with_translation = HashMap::new();

    for value in value_fields {

            let response = management_response::create_and_management_response(
                client, options_request, &value, languaje, pattern)
                .await.expect("msg");//TODO

                let first_value_in_body = response.1.first().expect("");
                json_value_with_translation.insert((*value).to_owned(),first_value_in_body.clone());
    }

    Ok(json_value_with_translation)
}