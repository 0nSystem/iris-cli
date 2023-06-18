use std::collections::HashMap;
use std::path::PathBuf;

use crate::cli::{Cli, ModeSql};
use crate::petitions::config_request::MultiplesApiParams;
use crate::system_resources::actions::{self, get_file_to_string};
use color_eyre::{Report, Result};
use log::info;

use self::text_procces::text_command;

mod json_procces;
mod sql_procces;
mod template_procces;
mod text_procces;

pub async fn start_procces(args_cli: &Cli) -> Result<()> {
    log::info!("Start procces");
    let output_map_name_to_add_file_and_info_template: HashMap<String, String> =
        match &args_cli.command {
            crate::cli::Commands::Template => template_procces::create_default_template(),
            _ => procces_modes_commands(args_cli).await,
        }?;

    export_result_in_file_or_print(
        &output_map_name_to_add_file_and_info_template,
        &args_cli.export,
    )?;

    log::info!("Finish procces");
    Ok(())
}

async fn procces_modes_commands(args_cli: &Cli) -> Result<HashMap<String, String>> {
    let path_config_file = &args_cli
        .config
        .as_ref()
        .ok_or(Report::msg("Require param config file"))?;

    info!("Reading config file {:?}", path_config_file);
    let config_file = serde_json::de::from_str(&actions::get_file_to_string(path_config_file)?)?;

    let language = args_cli
        .language
        .as_ref()
        .ok_or_else(|| Report::msg("Require param languaje"))?;

    let map_name_to_add_file_and_info_template = match &args_cli.command {
        crate::cli::Commands::Json { field_translate } => {
            info!("Command Json paths expresions: {:#?}", field_translate);
            procces_json(&args_cli.file, field_translate, language, &config_file).await
        }
        crate::cli::Commands::Sql { field_index, mode } => {
            info!("Command Sql mode: {}, field_index: {}", mode, field_index);
            procces_sql(&args_cli.file, field_index, mode, language, &config_file).await
        }
        crate::cli::Commands::Text { text_translate } => {
            info!("Command Text");
            procces_text(text_translate, &args_cli.file, language, &config_file).await
        }
        _ => Err(Report::msg("Not support operation")),
    }?;

    Ok(map_name_to_add_file_and_info_template)
}

async fn procces_sql(
    file: &Option<PathBuf>,
    field_index: &str,
    mode: &ModeSql,
    language: &str,
    config_file: &MultiplesApiParams,
) -> Result<HashMap<String, String>> {
    let text: String = actions::get_file_to_string(
        file.as_ref()
            .ok_or_else(|| Report::msg("Require param file"))?,
    )?;
    let index: Vec<usize> = field_index
        .split(',')
        .flat_map(|f| f.parse::<usize>())
        .collect();
    self::sql_procces::sql_command_with_multiples_api_params(
        &index,
        mode,
        &text,
        language,
        config_file,
    )
    .await
}

async fn procces_json(
    file: &Option<PathBuf>,
    field_translate: &Vec<String>,
    language: &str,
    config_file: &MultiplesApiParams,
) -> Result<HashMap<String, String>> {
    let text = actions::get_file_to_string(
        file.as_ref()
            .ok_or_else(|| Report::msg("Require param file"))?,
    )?;

    self::json_procces::json_command_with_multiples_api_params(
        field_translate,
        &text,
        language,
        config_file,
    )
    .await
}

async fn procces_text(
    text_translate_in_command: &Option<String>,
    text_file: &Option<std::path::PathBuf>,
    languaje: &str,
    config: &MultiplesApiParams,
) -> Result<HashMap<String, String>> {
    if let Some(text) = text_translate_in_command {
        text_command(text, languaje, config).await
    } else if let Some(text_path_file) = text_file {
        let file_string = get_file_to_string(text_path_file)?;
        text_command(&file_string, languaje, config).await
    } else {
        Err(Report::msg("Require text to translate"))
    }
}

fn export_result_in_file_or_print(
    output_map_name_to_add_file_and_info_template: &HashMap<String, String>,
    export_path: &Option<std::path::PathBuf>,
) -> Result<(), Report> {
    if let Some(file_export) = &export_path {
        for (tittle, text) in output_map_name_to_add_file_and_info_template {
            let file_name = file_export
                .file_name()
                .ok_or_else(|| Report::msg("Cant resolve filename"))?
                .to_str()
                .ok_or_else(|| Report::msg("Cant no conver path to str"))?;

            let mut new_file_name = String::new();
            new_file_name.push_str(tittle);
            new_file_name.push('_');
            new_file_name.push_str(file_name);

            let mut new_path = file_export.clone();
            new_path.set_file_name(new_file_name);

            //TODO
            actions::create_and_write_file(&new_path, text)?;
        }
    } else {
        output_map_name_to_add_file_and_info_template
            .iter()
            .for_each(|entry| {
                let output_name = entry.0;
                let output_result = entry.1;

                println!("\n\n\t{output_name}\n");
                println!("{output_result}\n");
            });
    }
    Ok(())
}
