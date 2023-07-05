pub mod client;
pub mod management_response;

#[cfg(test)]
pub mod translation {
    use crate::mock::{
        run_mock_with_params_in_url_response_body_translations, END_POINT_TRANSLATE,
        TEXT_ASSOCIATED_TRANSLATION_ES,
    };
    use iris::request::{
        client::{build_client, options_request_client::OptionClientRequest},
        config_request::ParamRequest,
        constants, translation as translation_text, translation_all_values,
    };
    use reqwest::Method;

    #[tokio::test]
    async fn translation() {
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

        let text_translated = translation_text(
            &client.unwrap(),
            &config_request,
            text,
            language,
            "$..translate",
        )
        .await;

        assert!(text_translated.is_ok(), "Error translation");
        assert_eq!(
            TEXT_ASSOCIATED_TRANSLATION_ES
                .get(text)
                .expect("Not found key in mock text map translations")
                .to_string(),
            text_translated.unwrap()
        );
    }

    #[tokio::test]
    async fn translation_all() {
        let text = String::from("Hello");
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

        let multiples_texts_to_translate = vec![text.clone()];
        let translations = translation_all_values(
            &client.unwrap(),
            &config_request,
            multiples_texts_to_translate.as_slice(),
            language,
            "$..translate",
        )
        .await;
        assert!(&translations.is_ok(), "Error translation");

        let first_translation = translations.as_ref().unwrap().iter().next();
        assert!(
            first_translation.is_some(),
            "Not found translations in translation_all_values"
        );

        assert_eq!(
            TEXT_ASSOCIATED_TRANSLATION_ES
                .get(text.as_str())
                .expect("Not found key in mock text map translations")
                .to_string(),
            first_translation.unwrap().1.to_owned()
        );
    }
}
