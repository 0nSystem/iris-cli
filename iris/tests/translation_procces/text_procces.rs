use iris::request::{
    config_request::{ApiParams, ParamRequest},
    constants,
};
use iris::translations_procces::text_procces;

use crate::mock::{run_mock_with_params_in_url_response_body_translations, END_POINT_TRANSLATE};

#[tokio::test]
async fn command_text() {
    let mock = run_mock_with_params_in_url_response_body_translations();

    let api_param = ApiParams {
        name: None,
        authentication: None,
        url: mock.url() + END_POINT_TRANSLATE,
        get_value_json: "$..translate".to_owned(),
        method_request: iris::request::config_request::MethodRequest::Get,
        params_request: vec![ParamRequest::InUri(format!(
            "text={}&language={}",
            constants::ENVIRONMENT_VARIABLE_TEXT_TRANSLATE,
            constants::ENVIRONMENT_VARIABLE_LANGUAGE_TRANSLATE
        ))],
    };

    let text_translated = text_procces::text_command("Hello", "ES", &api_param).await;
    assert!(
        text_translated.is_ok(),
        "Error text translate {:#?}",
        text_translated
    );
    assert_eq!("Hola", text_translated.unwrap());
}
