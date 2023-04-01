use serde::{Deserialize, Serialize};

pub struct ParamsRequest<'a> {
    pub api_key: &'a str,
    pub message: &'a Message<'a>,
}

pub struct Message<'a> {
    pub text: &'a str,
    pub cod_lang: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBodyDeepl {
    pub translations: Vec<Tranlation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tranlation {
    pub detected_source_language: String,
    pub text: String,
}

