use crate::cli::Cli;
use crate::system_resources::model::config_file::{ApiParams, ConfigFile};

mod json_procces;
mod sql_procces;
mod text_procces;


pub fn start_procces<'a>(config_file: &'a ConfigFile, args_cli: &'a Cli) {

    config_file.configurations.iter().for_each(|api_params| create_procces_from_api_params(api_params, args_cli))
}

pub fn create_procces_from_api_params(api_params: &ApiParams, args_cli: &Cli) {
    
    match &args_cli.command {
        crate::cli::Commands::Json { field_transalte } => todo!(),
        crate::cli::Commands::Sql { field_index_translate } => todo!(),
        crate::cli::Commands::Text { text_transalate } => todo!(),
    }


}
