use iris::request::{
    client::{build_client, options_request_client::OptionClientRequest},
    config_request::ParamRequest,
    constants,
    management_response::create_and_management_response,
};
use reqwest::Method;

use crate::mock::{
    run_mock_with_params_in_url_response_body_translations, END_POINT_TRANSLATE,
    TEXT_ASSOCIATED_TRANSLATION_ES,
};

#[tokio::test]
async fn create_and_management_response_returning_translation() {
    let text = "Hello";
    let language = "ES";
    let mock = run_mock_with_params_in_url_response_body_translations();

    let client = build_client(None);

    assert!(client.is_ok(), "Error building client {:#?}", &client);

    let config_request = OptionClientRequest {
        method_request: Method::GET,
        url: mock.url() + END_POINT_TRANSLATE,
        params_request: vec![ParamRequest::InUri(format!(
            "text={}&language={}",
            constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE,
            constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE
        ))],
    };

    let reponse = create_and_management_response(
        &client.unwrap(),
        &config_request,
        text,
        language,
        "$..translate",
    )
    .await;

    assert!(
        reponse.is_ok(),
        "Error create and management response {:#?}",
        reponse
    );

    assert_eq!(
        TEXT_ASSOCIATED_TRANSLATION_ES
            .get(text)
            .expect("Error getting text translation in mock map")
            .to_owned(),
        reponse.unwrap().1[0]
    )
}
