use crate::mock::{run_mock_with_params_in_url_response_body_translations, END_POINT_TRANSLATE};
use iris::{
    cli::ModeSql,
    request::{
        config_request::{ApiParams, ParamRequest},
        constants,
    },
    translations_procces::sql_procces::{self, get_text_to_translate_fields_queries_sql},
};

#[tokio::test]
async fn sql_command_insert() {
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

    let text = "INSERT INTO TABLE_NAME VALUES(1,'Hello','Bye');";
    let indexs: Vec<usize> = vec![1, 2];
    let query_translate =
        sql_procces::sql_command(&indexs, &ModeSql::Insert, text, "ES", &api_param).await;

    assert!(
        query_translate.is_ok(),
        "Error translation query {:#?}",
        query_translate
    );
    assert_eq!(
        "INSERT INTO TABLE_NAME VALUES(1,'Hola','Adios');",
        query_translate.unwrap()
    )
}

#[tokio::test]
async fn sql_command_update() {
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

    let text = "UPDATE TABLE_NAME SET FIRST=1,SECOND='Hello',THIRD='Bye');";
    let indexs: Vec<usize> = vec![1, 2];
    let query_translate =
        sql_procces::sql_command(&indexs, &ModeSql::Update, text, "ES", &api_param).await;

    assert!(
        query_translate.is_ok(),
        "Error translation query {:#?}",
        query_translate
    );
    assert_eq!(
        "UPDATE TABLE_NAME SET FIRST=1,SECOND='Hola',THIRD='Adios');",
        query_translate.unwrap()
    )
}

#[test]
fn get_text_in_sql_queries_insert() {
    let text = "INSERT INTO TABLE_NAME VALUES(1,'Hello','Bye');";
    let index: Vec<usize> = vec![1, 2];

    let values_captured =
        get_text_to_translate_fields_queries_sql(text, index.as_slice(), &ModeSql::Insert);

    assert!(
        values_captured.is_ok(),
        "Error getting values in query: {:#?}",
        values_captured
    );
    assert_eq!(vec!["Hello", "Bye"], values_captured.unwrap());
}

#[test]
fn get_text_in_sql_queries_insert_multiples() {
    let text = "INSERT INTO TABLE_NAME VALUES(1,'Hello','Bye'),(1,'Hello','Bye');";
    let index: Vec<usize> = vec![1, 2];

    let values_captured =
        get_text_to_translate_fields_queries_sql(text, index.as_slice(), &ModeSql::Insert);

    assert!(
        values_captured.is_ok(),
        "Error getting values in query, {:#?}",
        values_captured
    );
    assert_eq!(
        vec!["Hello", "Bye", "Hello", "Bye"],
        values_captured.unwrap()
    );
}
#[test]
fn get_text_in_sql_queries_insert_multiples_with_enter_separation() {
    let text = "INSERT INTO TABLE_NAME VALUES(1,'Hello','Bye'),\n(1,'Hello','Bye');";
    let index: Vec<usize> = vec![1, 2];

    let values_captured =
        get_text_to_translate_fields_queries_sql(text, index.as_slice(), &ModeSql::Insert);

    assert!(
        values_captured.is_ok(),
        "Error getting values in query: {:#?}",
        values_captured
    );
    assert_eq!(
        vec!["Hello", "Bye", "Hello", "Bye"],
        values_captured.unwrap()
    );
}

#[test]
fn get_text_in_sql_queries_update() {
    let text = "UPDATE TABLE_NAME SET FIRST=1,SECOND='Hello',THIRD='Bye');";
    let index: Vec<usize> = vec![1, 2];

    let values_captured =
        get_text_to_translate_fields_queries_sql(text, index.as_slice(), &ModeSql::Update);

    assert!(
        values_captured.is_ok(),
        "Error getting values in query: {:#?}",
        values_captured
    );
    assert_eq!(vec!["Hello", "Bye"], values_captured.unwrap());
}
