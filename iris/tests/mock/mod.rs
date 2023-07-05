use std::collections::HashMap;

use lazy_static::lazy_static;
use mockito::{Matcher, Server, ServerGuard};
use regex::Regex;
use serde_json::json;

pub const END_POINT_TRANSLATE: &str = "/translate";
pub const END_POINT_VALIDATION: &str = "/translate";
pub const VALUE_AUTORIZATION: &str = "api";

pub fn run_mock_with_params_in_url_empty_response(
    text: &str,
    language: &str,
    method: &str,
    reponse_status: usize,
) -> ServerGuard {
    let mut server = Server::new();

    server
        .mock(method, END_POINT_TRANSLATE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("text".to_owned(), text.to_owned()),
            Matcher::UrlEncoded("language".to_owned(), language.to_owned()),
        ]))
        .with_status(reponse_status)
        .create();

    server
        .mock("GET", END_POINT_VALIDATION)
        .match_header(reqwest::header::AUTHORIZATION.as_str(), VALUE_AUTORIZATION)
        .with_status(200)
        .create();

    server
}

lazy_static! {
    pub static ref TEXT_ASSOCIATED_TRANSLATION_ES: HashMap<&'static str, &'static str> =
        HashMap::from([("Hello", "Hola"), ("Bye", "Adios")]);
}

pub fn run_mock_with_params_in_url_response_body_translations() -> ServerGuard {
    let re = Regex::new(r"\?text=(?P<value>[^&]+)")
        .expect("Error build regex to get value in text field");

    let mut server = Server::new();

    server
        .mock("GET", END_POINT_TRANSLATE)
        .match_query(Matcher::Any)
        .with_status(200)
        .with_body_from_request(move |r| {
            if let Some(capture) = re.captures(r.path_and_query()) {
                if let Some(value) = capture.name("value") {
                    return json!({
                        "translate": TEXT_ASSOCIATED_TRANSLATION_ES
                        .get(value.as_str())
                        .expect("Not found value in mock to translate")
                        .to_string()
                    })
                    .to_string()
                    .into();
                }
            }
            json!({
                "translate": "Error"
            })
            .to_string()
            .into()
        })
        .create();

    server
}
