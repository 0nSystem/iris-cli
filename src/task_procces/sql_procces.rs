use std::{collections::HashMap, path::PathBuf};

use color_eyre::Report;

use crate::{
    cli::ModeSql,
    system_resources::model::config_file::{ApiParams, ConfigFile},
};

pub async fn config_and_run_sql_command<'a>(
    indexs: &'a Vec<String>,
    mode: &'a ModeSql,
    file: &'a str,
    language: &'a String,
    config_file: &'a ConfigFile,
) -> Result<HashMap<String, String>, Report> {
    let mut map_name_api_and_translation = HashMap::new();

    for (i, api_param) in config_file.configurations.iter().enumerate() {
        let name = api_param.name.clone().unwrap_or_else(|| i.to_string()); //TODO remove clone with if's

        map_name_api_and_translation.insert(
            name,
            run_sql_command(indexs, mode, language, api_param).await?,
        );
    }

    Ok(map_name_api_and_translation)
}

async fn run_sql_command<'a>(
    indexs: &'a Vec<String>,
    mode: &'a ModeSql,
    language: &'a String,
    config_file: &'a ApiParams,
) -> Result<String, Report> {
    Err(Report::msg("message"))
}
