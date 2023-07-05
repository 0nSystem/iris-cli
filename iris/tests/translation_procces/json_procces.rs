use iris::{
    request::{
        config_request::{ApiParams, ParamRequest},
        constants,
    },
    translations_procces::json_procces,
};
use serde_json::json;

use crate::mock::{run_mock_with_params_in_url_response_body_translations, END_POINT_TRANSLATE};

#[tokio::test]
async fn json_command() {
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

    let json_to_translate = json!({
        "a":"Hello",
        "b":{
            "c":"Bye"
        }
    });
    let pattern_expresions_to_capture_field_to_translate =
        vec!["$.a".to_owned(), "$..c".to_owned()];
    let translation = json_procces::json_command(
        &api_param,
        "ES",
        &json_to_translate,
        &pattern_expresions_to_capture_field_to_translate,
    )
    .await;

    assert!(
        translation.is_ok(),
        "Error translation json {:#?}",
        translation
    );
    let excepte_json = serde_json::to_string_pretty(&json!({
        "a":"Hola",
        "b":{
            "c":"Adios"
        }
    }));
    assert!(
        excepte_json.is_ok(),
        "Error building excepted json {:#?}",
        excepte_json
    );
    assert_eq!(excepte_json.unwrap(), translation.unwrap().trim());
}
