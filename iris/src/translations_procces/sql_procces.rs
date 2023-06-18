use std::collections::HashMap;

use color_eyre::Result;
use regex::Regex;

use crate::{
    cli::ModeSql,
    petitions::{
        self,
        client::options_request_client,
        config_request::{ApiParams, MultiplesApiParams},
        translation_all_values,
    },
};

pub async fn sql_command_with_multiples_api_params(
    indexs: &[usize],
    mode: &ModeSql,
    text: &str,
    language: &str,
    config_file: &MultiplesApiParams,
) -> Result<HashMap<String, String>> {
    let fields_to_translate = get_text_to_translate_fields_queries_sql(text, indexs, mode)?;

    let mut map_name_api_and_translation = HashMap::new();

    for (i, api_param) in config_file.configurations.iter().enumerate() {
        let name = api_param.name.clone().unwrap_or_else(|| i.to_string()); //TODO remove clone with if's

        map_name_api_and_translation.insert(
            name,
            sql_command(&fields_to_translate, text, language, api_param).await?,
        );
    }

    Ok(map_name_api_and_translation)
}

pub async fn sql_command(
    to_translation: &[String],
    text_file: &str,
    language: &str,
    api_param: &ApiParams,
) -> Result<String> {
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

    replace_text(translations, text_file)
}

fn get_text_to_translate_fields_queries_sql(
    text: &str,
    indexs: &[usize],
    mode: &ModeSql,
) -> Result<Vec<String>> {
    let string_join_with_all_fields_capture_in_query = match mode {
        ModeSql::Insert => get_text_to_translate_fields_queries_sql_insert(text),
        ModeSql::Update => get_text_to_translate_fields_queries_sql_update(text),
    }?;

    let fields = get_filter_fields_by_index_with_mode(
        string_join_with_all_fields_capture_in_query,
        indexs,
        mode,
    );

    Ok(fields)
}
fn get_text_to_translate_fields_queries_sql_update(text: &str) -> Result<Vec<String>> {
    //TODO actualmente el regex obliga a que sea un string
    let regex = Regex::new(r"(?i)([A-Z_]+)\s*=\s*('([^']*)'|(\d+))")?;

    let mut row = vec![];

    for line in text.lines() {
        let line_matches: Vec<String> = regex
            .find_iter(line)
            .map(|f| f.as_str().trim().to_string())
            .collect();
        row.push(line_matches.join(",").replace('\'', ""))
    }

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

    let text_join = row_prepared.join(",").replace([';', '\''], "");
    let regex = regex::Regex::new(r#"\([^)]+\)"#)?;
    let rows: Vec<&str> = regex
        .find_iter(&text_join)
        .map(|f| {
            let values = f.as_str().trim();
            &values[1..values.len() - 1]
        })
        .collect();

    let join_to_replace_brackets_and_others = rows.join(";"); //TODO ojo mirar si se puede cambiar el replace por otro regex

    let splitted_to_cast_vec = join_to_replace_brackets_and_others
        .split(';')
        .map(|f| f.to_string())
        .collect();

    Ok(splitted_to_cast_vec)
}

//El campo 0 tambien cuenta en principio
fn get_filter_fields_by_index_with_mode(
    rows: Vec<String>, //a,a,a,a or a=1,b=2
    indexs: &[usize],  //TODO
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

fn replace_text(translation: HashMap<&String, String>, text: &str) -> Result<String> {
    let mut new_text = text.to_string();
    for (old_value, new_value) in translation {
        let regex_replace = Regex::new(format!("'{}'", old_value).as_str())?;
        new_text = regex_replace
            .replace_all(&new_text, format!("'{}'", new_value))
            .to_string();
    }

    Ok(new_text)
}