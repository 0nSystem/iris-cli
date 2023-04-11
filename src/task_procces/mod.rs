use color_eyre::Report;
use reqwest::Response;
use serde_json::{Map, Value};

use crate::cli::{Cli, Commands};
use crate::petitions::management_response::{handle_error_petition_log, ErrorPetition};
use crate::system_resources::model::config_file::{ApiParams, ConfigFile};

use self::structs_params::CliParamsFormatProcces;

mod json_procces;
mod sql_procces;
mod text_procces;

pub async fn start_procces<'a>(config_file: &'a ConfigFile, args_cli: &'a Cli) {
    let params_format_procces = CliParamsFormatProcces::from(args_cli);

    for config in config_file.configurations.iter() {
        create_procces_from_api_params(config, &params_format_procces, &args_cli.command).await;
    }
}

async fn create_procces_from_api_params<'a>(
    api_params: &'a ApiParams,
    params_procces: &'a CliParamsFormatProcces<'a>,
    command: &'a Commands,
) {
    log::info!("Start procces to api request");
    match command {
        crate::cli::Commands::Json { field_translate } => todo!(),
        crate::cli::Commands::Sql {
            field_index_translate,
        } => todo!(),
        crate::cli::Commands::Text { text_translate } => {
            text_procces::send_petition_text_format(&text_translate, api_params, params_procces)
                .await;
        }
    }
}

mod structs_params {
    use reqwest::Method;

    use crate::{
        cli::Cli,
        system_resources::model::{
            config_file::ApiParams, options_request_client::OptionClientRequest,
        },
    };

    pub struct CliParamsFormatProcces<'a> {
        pub language: &'a str,
        pub file: &'a Option<String>,
        pub export: &'a Option<String>,
    }

    impl<'a> From<&'a Cli> for CliParamsFormatProcces<'a> {
        fn from(value: &'a Cli) -> Self {
            Self {
                language: value.language.as_str(),
                file: &value.file,
                export: &value.export,
            }
        }
    }

    impl<'a> From<&'a ApiParams> for OptionClientRequest<'a> {
        fn from(value: &'a ApiParams) -> Self {
            Self {
                method_request: Method::from(&value.method_request),
                url: &value.url,
                params_request: &value.params_request,
            }
        }
    }
}
