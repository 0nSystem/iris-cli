use mockito::{Matcher, Server, ServerGuard};

pub const END_POINT_TRANSLATE: &str = "/translate";
pub const END_POINT_VALIDATION: &str = "/translate";
pub const VALUE_AUTORIZATION: &str = "api";

pub fn run_mock_with_params_in_url(
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
