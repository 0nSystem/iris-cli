use std::collections::HashMap;

use crate::cli::Cli;
use crate::system_resources::actions;
use color_eyre::Report;
use log::info;

mod json_procces;
mod sql_procces;
mod template_procces;
mod text_procces;

pub async fn start_procces(args_cli: &Cli) -> Result<(), Report> {
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

async fn procces_modes_commands(args_cli: &Cli) -> Result<HashMap<String, String>, Report> {
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
            let text = actions::get_file_to_string(
                args_cli
                    .file
                    .as_ref()
                    .ok_or_else(|| Report::msg("Require param file"))?,
            )?; //TODO

            self::json_procces::config_and_run_json_command(
                field_translate,
                &text,
                language,
                &config_file,
            )
            .await
        }
        crate::cli::Commands::Sql { field_index, mode } => {
            info!("Command Sql mode: {}, field_index: {}", mode, field_index);
            let text = actions::get_file_to_string(
                args_cli
                    .file
                    .as_ref()
                    .ok_or_else(|| Report::msg("Require param file"))?,
            )?; //TODO
            self::sql_procces::config_and_run_sql_command(
                field_index,
                mode,
                &text,
                language,
                &config_file,
            )
            .await
        }
        crate::cli::Commands::Text { text_translate } => {
            info!("Command Text");
            self::text_procces::config_and_run_text_command(
                text_translate,
                &args_cli.file,
                language,
                &config_file,
            )
            .await
        }
        _ => todo!(),
    }?;
    //TODO
    Ok(map_name_to_add_file_and_info_template)
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
