#[cfg(test)]
mod build_client {
    use iris::request::client::build_client;
    #[test]
    fn with_authentication() {
        let client = build_client(Some(&"api_key".to_owned()));

        assert!(client.is_ok(), "Error create client with authentication");
    }
    #[test]
    fn not_authentication() {
        let client = build_client(None);

        assert!(client.is_ok(), "Error create client not authentication");
    }
}

#[cfg(test)]
mod build_request {
    use iris::request::constants;
    use iris::request::{
        client::{build_request, options_request_client::OptionClientRequest},
        config_request::ParamRequest,
    };
    use reqwest::Method;

    #[test]
    fn build_request_with_replace_params_in_url_as_form() {
        let config_request = OptionClientRequest {
            method_request: Method::GET,
            url: "localhost:8080".to_owned(),
            params_request: vec![ParamRequest::InUri(format!(
                "text={}&language={}",
                constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE,
                constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE
            ))],
        };

        let text = "Hello";
        let language = "ES";
        let request = build_request(&config_request, text, language);

        assert!(request.is_ok(), "Error build request");

        assert!(
            request.as_ref().unwrap().url().to_string().contains(text),
            "Error replace text in url params"
        );
        assert!(
            request
                .as_ref()
                .unwrap()
                .url()
                .to_string()
                .contains(language),
            "Error replace language in url params"
        );
    }

    //Pending in body, now not supported
}

#[cfg(test)]
mod send_request {

    use iris::request::{
        client::{
            build_client, build_request, options_request_client::OptionClientRequest, send_request,
        },
        config_request::ParamRequest,
        constants,
    };
    use reqwest::Method;

    use crate::mock::{
        run_mock_with_params_in_url_empty_response, END_POINT_TRANSLATE, END_POINT_VALIDATION,
        VALUE_AUTORIZATION,
    };

    #[tokio::test]
    async fn send_request_params_in_url() {
        let text = "Hello";
        let language = "ES";
        let mock = run_mock_with_params_in_url_empty_response(text, language, "GET", 200);

        let config_request = OptionClientRequest {
            method_request: Method::GET,
            url: mock.url() + END_POINT_TRANSLATE,
            params_request: vec![ParamRequest::InUri(format!(
                "text={}&language={}",
                constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE,
                constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE
            ))],
        };

        let client = build_client(None);

        assert!(client.is_ok(), "Error build client {:#?}", client);
        let request = build_request(&config_request, text, language);
        assert!(
            request.as_ref().is_ok(),
            "Error build request {:#?}",
            request
        );

        let response = send_request(&client.unwrap(), request.unwrap()).await;

        assert!(
            response.is_ok(),
            "Error execution send request {:#?}",
            response
        );
    }

    #[tokio::test]
    async fn send_request_with_validation() {
        let text = "Hello";
        let language = "ES";
        let mock = run_mock_with_params_in_url_empty_response(text, language, "GET", 200);

        let config_request = OptionClientRequest {
            method_request: Method::GET,
            url: mock.url() + END_POINT_VALIDATION,
            params_request: vec![],
        };

        let client = build_client(Some(&VALUE_AUTORIZATION.to_owned()));

        assert!(client.is_ok(), "Error build client {:#?}", client);
        let request = build_request(&config_request, text, language);
        assert!(
            request.as_ref().is_ok(),
            "Error build request {:#?}",
            request
        );

        let response = send_request(&client.unwrap(), request.unwrap()).await;

        assert!(
            response.is_ok(),
            "Error execution send request {:#?}",
            response
        );
    }
}
