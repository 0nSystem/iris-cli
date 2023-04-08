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

pub mod options_builder_client {
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};

    use super::config_file::ApiParams;

    impl<'a> From<&'a ApiParams> for HeaderMap {
        fn from(value: &ApiParams) -> Self {
            let mut headers = HeaderMap::new();

            if value.authentication.is_some() {
                let parsed_authentication = get_authentication_mode(
                    value
                        .authentication
                        .as_ref()
                        .expect("Error getting authentication mode")
                        .as_str(),
                );
                headers.append(parsed_authentication.0, parsed_authentication.1);
            }

            headers
        }
    }

    fn get_authentication_mode<'a>(authentication: &'a str) -> (HeaderName, HeaderValue) {
        (
            AUTHORIZATION,
            authentication
                .parse()
                .expect("Error parse api key to header value"),
        )
    }
}

pub mod options_request_client {
    use reqwest::{Method, Url};

    use super::config_file::{MethodRequest, ParamRequest};

    pub struct OptionClientRequest {
        pub method_request: Method,
        pub url: String,
        pub params_request: Vec<ParamRequest>,
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
