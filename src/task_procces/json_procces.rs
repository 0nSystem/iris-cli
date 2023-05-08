use std::collections::HashMap;

use color_eyre::Report;
use jsonpath_lib::{select, JsonPathError, selector, SelectorMut, Selector};
use serde_json::{json, Value};

use crate::{system_resources::{model::config_file::{ConfigFile, ApiParams}, actions}, task_procces::TaskError};
use crate::petitions::client;


pub async fn config_and_run_json_command<'a>(
    fields_to_translate: &'a Vec<String>, //pattern expression 
    text_file: &'a Option<std::path::PathBuf>,
    languaje: &'a str,
    config: &'a ConfigFile,
) -> Result<HashMap<String, String>, super::TaskError> {
    let path_text_field_validated = text_file.as_ref().ok_or_else(||TaskError::RequireField("Input Text".to_owned()))?;
    let file_json_in_str = actions::get_file_to_string(&path_text_field_validated).map_err(|_|TaskError::ReadFile)?;
    
    let file_json = json!(file_json_in_str); 

    for (i,conf) in config.configurations.iter().enumerate() {
        json_command_procces(conf, languaje,&file_json, fields_to_translate).await;    
    }

    todo!()
    
}

async fn json_command_procces<'a>(api_param: &'a ApiParams,languaje: &'a str,json_file: &'a Value, pattern_expresion: &'a Vec<String>) -> Result<(), Report>{
    let client = client::build_client::build_client(api_param.authentication.as_ref());

    let mut map_file_json_path_expresion_with_string_values: HashMap<&String,Vec<&Value>> = HashMap::new(); 
    let selector_json_file = jsonpath_lib::selector(json_file);//TODO

    for expresion in  pattern_expresion{
        find_by_expresion_and_parse_to_str(selector_json_file., &expresion);     
    }

    

    todo!()

}


//TODO change
fn find_by_expresion_and_parse_to_str<'a:'b,'b, T:FnMut(&str) -> Result<Vec<&'a Value>, JsonPathError> >(selector: T, pattern_expresion: &'a str) -> Result<Vec<&'b str>, Report>{
    let mut collections_values_parsed = Vec::new();
    for a in selector(pattern_expresion)? {
        if let Some(parser_value) = a.as_str() {
            collections_values_parsed.push(parser_value);
        }else{
            //Error path expresion
            todo!("Error path expresion")
        }
    }
    
    Ok(collections_values_parsed)
}