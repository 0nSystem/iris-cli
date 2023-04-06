use clap::Parser;
use color_eyre::{eyre::Result, Report};
use serde_json::Value;
use translator_cli::cli;
use translator_cli::{
    petitions::{deelp, handle_error_petition_log},
    system_resources::{
        actions::get_file, management_errors::handle_error_system_resources_log, model::ConfigFile,
    },
    utils::logger::config_logger,
};

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    //cli::Cli::parse_from(vec!["","--help"]);
    let arg_cli = cli::Cli::parse_from(vec!["", "-vv","-c","./config_file.json","-l","ES","text","Hello World"]);
    config_logger(arg_cli.verbose, env_logger::Target::Stdout).expect("Error config logger");
    
    log::info!("Starting translation-cli");
    log::debug!("Debug Message");

    let config_file = "./config_file.json";
    match get_file(config_file) {
        Ok(readed_file) => {
            let config_file = serde_json::from_slice::<ConfigFile>(&readed_file).expect(&format!("Error parse config file {}",config_file));

            let client_deepl =
                deelp::actions::build_client_to_deepl(&config_file.api_key_deepl.unwrap());

            let message = deelp::model::Message {
                cod_lang: "ES",
                text: "Hello World",
            };

            match deelp::actions::send_petition(&client_deepl, &message).await {
                Ok(a) => {
                    let json_strutc:Value = serde_json::from_slice(&a.bytes().await.unwrap()).unwrap();
                    let map = json_strutc.as_object().unwrap();
                    for entry in map {
                        println!("{} - {}",entry.0,entry.1)
                    }
                }
                Err(e) => handle_error_petition_log(&e),
            };
        }
        Err(error) => handle_error_system_resources_log(&error),
    };

    Ok(())
}
