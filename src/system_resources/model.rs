pub mod config_file {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigFile {
        pub configurations: Vec<ApiParams>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApiParams {
        pub name: Option<String>,
        pub method_request: MethodRequest,
        pub url: String,
        pub authentication: Option<String>,
        pub params_request: Vec<ParamRequest>,
        pub get_values_json: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ParamRequest {
        InUri(String),
        InBody(serde_json::Value),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum MethodRequest {
        Get = 0,
        Post = 1,
    }
}

pub mod options_request_client {
    use reqwest::Method;

    use super::config_file::{MethodRequest, ParamRequest};

    pub struct OptionClientRequest<'a> {
        pub method_request: Method,
        pub url: &'a str,
        pub params_request: &'a Vec<ParamRequest>,
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
