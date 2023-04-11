
use std::collections::HashMap;

use reqwest::{Error, Response, StatusCode};
use serde_json::{Value, json};

#[derive(Debug)]
pub enum ErrorPetition {
    StatusResponseNotValid(u16),
    ErrorSendRequest(String),
}

pub fn handle_error_petition_log(error: &ErrorPetition) {
    match error {
        ErrorPetition::StatusResponseNotValid(number) => {
            log::error!("Failed response with status: {number}")
        }
        ErrorPetition::ErrorSendRequest(url) => log::error!("Failed send request in {url}"),
    }
}

pub fn validate_status_response(response: &Response) -> Result<(), ErrorPetition> {
    match response.status() {
        StatusCode::OK | StatusCode::ACCEPTED => Ok(()),
        _ => Err(ErrorPetition::StatusResponseNotValid(
            response.status().as_u16(),
        )),
    }
}

#[derive(Debug,PartialEq)]
pub enum Node<T>{
    Collection(Box<Vec<Node<T>>>),
    Value(T),
    None,
    Null
}


//TODO is object contains multiples transaltions
pub async fn management_response<'a>(
    result_reponse: Result<Response, ErrorPetition>,
    key_to_get_values_json: &'a Vec<String>,
) -> Result<Node<Value>,()> {

    match result_reponse {
        Ok(response) => {
            let json_result: Result<serde_json::Value, Error> = response.json().await;
            if let Ok(json_value) = json_result {
                return Ok(management_value(json!({
                    "translations":[
                        {
                            "text": "ASDASD",
                            "detected_source_language":"ES"
                        },
                        {
                            "text": "ASDASD",
                            "detected_source_language":"ES"
                        }
                    ]
                }), key_to_get_values_json,0));
            }
        }
        Err(error) => {
            handle_error_petition_log(&error);
        }
    }
    Err(()) //TODO make enum to error response this function

}
//TODO change struct not support is body isnt object
//ROOT JSON
fn management_value<'a>(
    json_response: serde_json::Value,
    keys_to_get_values_json: &'a Vec<String>,
    index_key_start: usize //TODO can remove this param
) -> Node<Value> {

    match json_response {
        Value::Object(_) => {
            let mut map_value = Box::new(Vec::new());
            for key in keys_to_get_values_json {
                let key_to_find_splitted: Vec<&str> = key.split(".").collect();
                map_value.push(filter_value_with_key(&key_to_find_splitted, &index_key_start, &json_response));
            }
            Node::Collection(map_value)
        }
        Value::Array(json_array) => todo!(),
        Value::Null => Node::Null,
        other_initial_type => todo!()
    }
}



fn filter_value_with_key<'a>(key_by_find: &'a Vec<&'a str>, index_key_start: &'a usize, json_value: &'a Value) -> Node<Value> {
    match json_value {
        Value::Array(array_value) => {
            if validate_end_node_by_key(key_by_find, &index_key_start) {
                let mut conversion_array = Box::new(Vec::new());
                array_value.iter().for_each(|value_array| conversion_array.push(filter_value_with_key(key_by_find, index_key_start, value_array)));
                return Node::Collection(conversion_array);
            }else {
                let new_key_index_start = index_key_start + 1;
                let mut collection_find_values = Box::new(Vec::new());
                array_value.iter().for_each(|value_json_array|collection_find_values.push(filter_value_with_key(key_by_find, &new_key_index_start, value_json_array)));
                return Node::Collection(collection_find_values);
            }
        },
        Value::Object(json_map) => {
            if let Some(found_value) = json_map.get(key_by_find[*index_key_start]) {
                if validate_end_node_by_key(key_by_find,&index_key_start) {
                    return filter_value_with_key(key_by_find, index_key_start, found_value);
                }else{
                    let new_key_index_start = index_key_start + 1 ; 
                    return filter_value_with_key(key_by_find, &new_key_index_start, found_value)
                }
            } else {
                Node::None
            }
            
        },
        Value::Null => Node::Null,
        other_type_value => Node::Value(other_type_value.clone()),
    }

}

fn validate_end_node_by_key<'a>(key_by_find: &'a Vec<&'a str>, key_index_start: &'a usize) -> bool{
    &(key_by_find.len()-1) == key_index_start
}