use std::collections::HashMap;

use color_eyre::{Report, Result};
use regex::Regex;

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
) -> Result<Vec<String>> {
    let string_join__with_all_fields_capture_in_query = match mode {
        ModeSql::Insert => get_text_to_translate_fields_queries_sql_insert(text),
        ModeSql::Update => get_text_to_translate_fields_queries_sql_update(text),
    }?;

    println!(
        "result parse queries: {:?}",
        string_join__with_all_fields_capture_in_query
    );
    let fields = get_filter_fields_by_index_with_mode(
        string_join__with_all_fields_capture_in_query,
        indexs,
        mode,
    );

    println!("fields found by index: {:?}", fields);

    Ok(fields)
}
fn get_text_to_translate_fields_queries_sql_update(text: &str) -> Result<Vec<String>> {
    //TODO actualmente el regex obliga a que sea un string
    let regex = Regex::new(r"\s+\w+\s*=\s*'([^']*)'")?;

    let mut row = vec![];

    for line in text.lines() {
        let line_matches: Vec<String> = regex
            .find_iter(line)
            .map(|f| f.as_str().to_string())
            .collect();
        row.push(line_matches.join(","))
    }

    println!("update rows: {:?}", row);
    Ok(row)
}
fn get_text_to_translate_fields_queries_sql_insert(text: &str) -> Result<Vec<String>> {
    let mut row_prepared: Vec<&str> = vec![];
    for line in text.lines() {
        if line.contains("VALUES") {
            let splitted_values_tag: Vec<&str> = line.split("VALUES").collect();
            row_prepared.extend(&splitted_values_tag[1..]);
        } else {
            row_prepared.push(line);
        }
    }

    let text_join = row_prepared.join(",").replace(';', "");
    let regex = regex::Regex::new(r#"\(([^)]+)\)"#)?;
    let rows: Vec<&str> = regex
        .find_iter(&text_join)
        .map(|f| f.as_str()) //TODO replace
        .collect();

    let join_to_replace_brackets_and_others = rows.join(";").replace(['(', ')'], ""); //TODO ojo mirar si se puede cambiar el replace por otro regex

    let splitted_to_cast_vec = join_to_replace_brackets_and_others
        .split(';')
        .map(|f| f.to_string())
        .collect();

    Ok(splitted_to_cast_vec)
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
            .filter_map(|f| f.split('=').nth(1).map(|str| str.trim().to_string())) //TODO
            .collect();
        return fields_translate_if_is_update;
    }

    rows_fields_splitted_and_filter_by_index
}

fn replace_text(translation: HashMap<&String, String>, text: &str) -> String {
    let mut new_text = text.to_string();
    for (old_value, new_value) in translation {
        println!("old_value: {:?}", old_value);
        println!("new_value: {:?}", new_value);
        new_text = new_text
            .replace(old_value.trim(), &format!("'{}'", new_value))
            .to_string();
        println!("new_text: {:?}", new_text);
    }

    new_text
}
