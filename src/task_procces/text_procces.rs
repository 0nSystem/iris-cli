use color_eyre::Report;

use crate::{
    petitions::{client, management_response::management_response},
    system_resources::{
        self,
        model::{config_file::ApiParams, options_request_client::OptionClientRequest},
    },
};

use super::structs_params::CliParamsFormatProcces;

pub async fn send_petition_text_format<'a>(
    text_translate: &Option<String>,
    api_params: &'a ApiParams,
    params_text_format: &'a CliParamsFormatProcces<'a>,
) {
    let client = client::build_client::build_client(api_params.authentication.as_ref());
    let options_client_request = OptionClientRequest::from(api_params);
    //If text translate in cli exist ignore file param in cli
    match text_translate {
        Some(text) => {
            let request = client::build_request::build_request(
                &options_client_request,
                text,
                params_text_format.language,
            );

            let response = client::send_request::send_request(&client, request).await;
            let result_found_nodes = management_response(response, &api_params.get_values_json).await;

            if let Ok(found_nodes) = result_found_nodes {
                println!("{:?}",found_nodes);
            }else {
                println!("Not found nodes");
            }
            
        }
        None => match params_text_format.file {
            Some(file_name) => {
                //TODO limit size chars?
                match system_resources::actions::get_file(&file_name) {
                    Ok(file_value) => todo!(),
                    Err(_) => todo!(),
                }
            }
            None => {
                panic!("Text translate in cli arguments and filename is empty")
            }
        },
    }
    // Todo add other procces
}
