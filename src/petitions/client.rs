pub mod build_client {
    use reqwest::{
        header::{HeaderMap, AUTHORIZATION},
        Client,
    };

    pub fn build_client<'a>(authentication: Option<&'a String>) -> Client {
        let mut headers = HeaderMap::new();
        if authentication.is_some() {
            headers.append(
                AUTHORIZATION,
                authentication
                    .expect("Not found Value")
                    .parse()
                    .expect("Error parse field headers authorization header value"),
            );
        }

        let build_client = Client::builder().default_headers(headers);

        build_client.build().expect("Error build client")
    }
}

pub mod build_request {
    use reqwest::Request;

    use crate::{
        petitions::constants, system_resources::model::options_request_client::OptionClientRequest,
    };

    use super::utils_client::{
        contains_environments_variables_in_body, contains_environments_variables_in_url,
    };

    //TODO replace body params

    pub fn build_request<'a>(
        options_client_request: &'a OptionClientRequest,
        text: &'a str,
        language: &'a str,
    ) -> Request {
        let mut request_builder = Request::new(
            options_client_request.method_request.clone(),
            options_client_request
                .url
                .parse()
                .expect("Error parse url str to struct URL"),
        );

        options_client_request
            .params_request
            .iter()
            .for_each(|params| match params {
                crate::system_resources::model::config_file::ParamRequest::InUri(params) => {
                    if contains_environments_variables_in_url(params) {
                        let params_replace = params
                            .replace(constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE, language)
                            .replace(constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE, text);
                        request_builder.url_mut().set_query(Some(&params_replace))
                    }
                }
                crate::system_resources::model::config_file::ParamRequest::InBody(params) => {
                    let map_object_body_params = params.as_object();
                    if contains_environments_variables_in_body(map_object_body_params) {
                        //TODO
                    }
                }
            });

        request_builder
    }
}

pub mod send_request {
    use reqwest::{Client, Request, Response};

    use crate::petitions::management_response::{validate_status_response, ErrorPetition};

    pub async fn send_request<'a>(
        client: &'a Client,
        request: Request,
    ) -> Result<Response, ErrorPetition> {
        let response_result = client.execute(request).await;

        match response_result {
            Ok(response) => {
                validate_status_response(&response)?;
                Ok(response)
            }
            Err(error) => Err(ErrorPetition::ErrorSendRequest(error.to_string())),
        }
    }
}

mod utils_client {
    use serde_json::{Map, Value};

    use crate::petitions::constants;

    pub fn contains_environments_variables_in_url<'a>(params: &'a str) -> bool {
        params.contains(constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE)
            || params.contains(constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE)
    }

    //TODO
    pub fn contains_environments_variables_in_body<'a>(
        body: Option<&'a Map<String, Value>>,
    ) -> bool {
        false
    }
}
