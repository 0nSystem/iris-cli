pub mod build_client {
    use color_eyre::{Report, Result};
    use reqwest::{
        header::{HeaderMap, AUTHORIZATION},
        Client,
    };

    pub fn build_client(authentication: Option<&String>) -> Result<Client> {
        let mut headers = HeaderMap::new();
        if let Some(auth) = authentication {
            headers.append(
                AUTHORIZATION,
                auth.parse()
                    .map_err(|_| Report::msg("Error parser Authentication header value"))?,
            );
        }

        let build_client = Client::builder().default_headers(headers);
        Ok(build_client.build()?)
    }
}

pub mod build_request {
    use color_eyre::Result;
    use reqwest::Request;

    use crate::petitions::{config_request, constants};

    use super::{
        options_request_client::OptionClientRequest,
        utils_client::{
            contains_environments_variables_in_body, contains_environments_variables_in_url,
        },
    };

    //TODO replace body params
    pub fn build_request<'a>(
        options_client_request: &'a OptionClientRequest,
        text: &'a str,
        language: &'a str,
    ) -> Result<Request> {
        let mut request_builder = Request::new(
            options_client_request.method_request.clone(),
            options_client_request.url.parse()?, //TODO
        );

        options_client_request
            .params_request
            .iter()
            .for_each(|params| match params {
                config_request::ParamRequest::InUri(params) => {
                    if contains_environments_variables_in_url(params) {
                        let params_replace = params
                            .replace(constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE, language)
                            .replace(constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE, text);
                        request_builder.url_mut().set_query(Some(&params_replace))
                    }
                }
                config_request::ParamRequest::InBody(params) => {
                    let map_object_body_params = params.as_object();
                    if contains_environments_variables_in_body(map_object_body_params) {
                        todo!()
                    }
                }
            });

        Ok(request_builder)
    }
}

pub mod send_request {
    use color_eyre::{Report, Result};
    use reqwest::{Client, Request, Response, StatusCode};

    pub async fn send_request(client: &Client, request: Request) -> Result<Response> {
        let response = client.execute(request).await?;
        validate_status_response(&response)?;
        Ok(response)
    }
    fn validate_status_response(response: &Response) -> Result<()> {
        match response.status() {
            StatusCode::OK | StatusCode::ACCEPTED => Ok(()),
            _ => Err(Report::msg("Status code reponse not valid")),
        }
    }
}

mod utils_client {
    use serde_json::{Map, Value};

    use crate::petitions::constants;

    pub fn contains_environments_variables_in_url(params: &str) -> bool {
        params.contains(constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE)
            || params.contains(constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE)
    }

    //TODO
    pub fn contains_environments_variables_in_body(_body: Option<&Map<String, Value>>) -> bool {
        todo!()
    }
}

pub mod options_request_client {
    use reqwest::Method;

    use crate::petitions::config_request::{ApiParams, MethodRequest, ParamRequest};

    #[derive(Clone)]
    pub struct OptionClientRequest {
        pub method_request: Method,
        pub url: String,
        pub params_request: Vec<ParamRequest>,
    }
    impl From<&ApiParams> for OptionClientRequest {
        fn from(value: &ApiParams) -> Self {
            Self {
                method_request: Method::from(&value.method_request),
                url: value.url.clone(),
                params_request: value.params_request.clone(),
            }
        }
    }

    impl<'a> From<&'a MethodRequest> for reqwest::Method {
        fn from(value: &'a MethodRequest) -> Self {
            match value {
                MethodRequest::Get => Method::GET,
                MethodRequest::Post => Method::POST,
            }
        }
    }
}