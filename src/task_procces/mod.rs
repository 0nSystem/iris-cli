use std::collections::HashMap;

use crate::cli::Cli;
use crate::system_resources::{actions, management_errors, model};

mod template_procces;
mod text_procces;

pub async fn start_procces<'a>(args_cli: &'a Cli) -> Result<(), TaskError> {
    log::info!("Start procces to api request");
    let output_map_name_to_add_file_and_info_template: HashMap<String, String> =
        match &args_cli.command {
            crate::cli::Commands::Template => template_procces::create_default_template(),
            _ => procces_modes_commands(&args_cli).await,
        }?;

    export_result_in_file_or_print(
        &output_map_name_to_add_file_and_info_template,
        &args_cli.export,
    );

    log::info!("Finish procces to api request");
    Ok(())
}

async fn procces_modes_commands<'a>(
    args_cli: &'a Cli,
) -> Result<HashMap<String, String>, TaskError> {
    let mut map_name_to_add_file_and_info_template = HashMap::new();

    let config_file: model::config_file::ConfigFile = match &args_cli.config {
        Some(path_config) => {
            let readed_config =
                actions::get_file_to_string(path_config).map_err(|_| TaskError::ReadFile)?;
            Ok(serde_json::de::from_str(&readed_config)
                .map_err(|_| TaskError::CantParseConfigFile)?)
        }
        None => Err(TaskError::RequireConfigFile),
    }?;

    Ok(map_name_to_add_file_and_info_template)
}

fn export_result_in_file_or_print<'a>(
    output_map_name_to_add_file_and_info_template: &'a HashMap<String, String>,
    export_path: &Option<std::path::PathBuf>,
) {
    if let Some(file_export) = &export_path {
        //Quitar lambda para hacer result
        output_map_name_to_add_file_and_info_template
            .iter()
            .for_each(|entry| {
                let new_path: std::path::PathBuf = match file_export
                    .file_name()
                    .ok_or_else(|| TaskError::ErrorWriteFile)
                {
                    Ok(file_name) => match file_name.to_str() {
                        Some(file_name_string) => {
                            //TODO can change this logic in other proccess to make new paths
                            let mut new_file_name = String::new();
                            new_file_name.push_str(entry.0);
                            new_file_name.push_str(file_name_string);

                            let mut clone_path_to_change_file_name = file_export.clone();
                            clone_path_to_change_file_name.set_file_name(new_file_name);
                            clone_path_to_change_file_name
                        }
                        None => file_export.clone(),
                    },
                    Err(_) => file_export.clone(),
                };

                let result_create_file = actions::create_and_write_file(&new_path, &entry.1)
                    .map_err(|error| {
                        TaskError::ErrorWriteFile(
                            management_errors::handle_error_system_resources_log(&error),
                        )
                    });

                /*
                match result_create_file {
                Ok(_) => todo!(),
                Err(_) => todo!(),
                }
                */
            });
    } else {
        output_map_name_to_add_file_and_info_template
            .iter()
            .for_each(|entry| {
                let output_name = entry.0;
                let output_result = entry.1;

                println!("\n\n\n\t {output_name}");
                println!("{output_result}");
            });
    }
}

//TODO change params
//TODO move cant access other modules
pub enum TaskError {
    ErrorRequireField(String),
    ErrorCreateTemplate,
    ErrorWriteFile(String),
    ReadFile,
    ErrorRequest,
    RequireConfigFile,
    CantParseConfigFile,
}

struct ParamsArgumentsAndConfig {



}



