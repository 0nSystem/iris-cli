use std::collections::HashMap;

use color_eyre::Report;

use crate::{
    cli::ModeSql,
    petitions::{self, client::options_request_client, translation_all_values},
    system_resources::model::config_file::{ApiParams, ConfigFile},
};

pub async fn config_and_run_sql_command(
    indexs: &str,
    mode: &ModeSql,
    text: &str,
    language: &str,
    config_file: &ConfigFile,
) -> Result<HashMap<String, String>, Report> {
    let usize_index = indexs.split(',').flat_map(|f| f.parse::<usize>()).collect();

    let fields_to_translate = get_text_to_translate_fields_queries_sql(text, &usize_index, mode)?;

    let mut map_name_api_and_translation = HashMap::new();

    for (i, api_param) in config_file.configurations.iter().enumerate() {
        let name = api_param.name.clone().unwrap_or_else(|| i.to_string()); //TODO remove clone with if's

        map_name_api_and_translation.insert(
            name,
            run_sql_command(&fields_to_translate, text, language, api_param).await?,
        );
    }

    Ok(map_name_api_and_translation)
}

async fn run_sql_command(
    to_translation: &Vec<String>,
    text_file: &str,
    language: &str,
    api_param: &ApiParams,
) -> Result<String, Report> {
    let client = petitions::client::build_client::build_client(api_param.authentication.as_ref())?;
    let options_request_client = options_request_client::OptionClientRequest::from(api_param);

    let translations = translation_all_values(
        &client,
        &options_request_client,
        to_translation,
        language,
        api_param.get_value_json.as_str(),
    )
    .await?;

    Ok(replace_text(translations, text_file))
}

fn get_text_to_translate_fields_queries_sql(
    text: &str,
    indexs: &Vec<usize>,
    mode: &ModeSql,
) -> Result<Vec<String>, Report> {
    //TODO tratar espacios
    let regex = match mode {
        ModeSql::Insert => regex::Regex::new(r"\((.*)\)*"),
        ModeSql::Update => regex::Regex::new("SET *"),
    }?;

    let rows: Vec<String> = regex
        .find_iter(text)
        .map(|f| f.as_str().to_string().replace(';', "")) //TODO replace
        .collect();

    let fields = get_filter_fields_by_index_with_mode(rows, indexs, mode);

    Ok(fields)
}

//El campo 0 tambien cuenta en principio
fn get_filter_fields_by_index_with_mode(
    rows: Vec<String>,   //a,a,a,a or a=1,b=2
    indexs: &Vec<usize>, //TODO
    mode: &ModeSql,
) -> Vec<String> {
    let mut rows_fields_splitted_and_filter_by_index = vec![];

    for row in rows {
        let row_fields_split_and_filter: Vec<String> = row
            .split(',')
            .enumerate()
            .filter_map(|f| {
                if indexs.contains(&f.0) {
                    Some(f.1.to_string()) //TODO
                } else {
                    None
                }
            })
            .collect();
        rows_fields_splitted_and_filter_by_index.extend(row_fields_split_and_filter);
    }

    if ModeSql::Update == *mode {
        //TODO
        let fields_translate_if_is_update = rows_fields_splitted_and_filter_by_index
            .iter()
            .filter_map(|f| f.split('=').nth(2).map(|str| str.to_string())) //TODO
            .collect();
        return fields_translate_if_is_update;
    }

    rows_fields_splitted_and_filter_by_index
}

fn replace_text(translation: HashMap<&String, String>, text: &str) -> String {
    let mut new_text = text.to_string();
    for (old_value, new_value) in translation {
        new_text = text.replace(&old_value.clone(), &new_value).to_string();
    }

    new_text
}
