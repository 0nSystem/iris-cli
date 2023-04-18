use clap::Parser;
use color_eyre::{eyre::Result, Report};
use serde_json::json;
use iris_cli::{cli, task_procces};
use iris_cli::{
    system_resources::{
        actions::get_file, management_errors::handle_error_system_resources_log, model::config_file,
    },
    utils::logger::config_logger,
};

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    //cli::Cli::parse_from(vec!["","--help"]);
    let arg_cli = cli::Cli::parse_from(vec![
        "",
        "-vv",
        "-c",
        "./config_file.json",
        "-l",
        "ES",
        "text",
        "Hello World",
    ]);
    config_logger(arg_cli.verbose, env_logger::Target::Stdout).expect("Error config logger");

    log::info!("Starting translation-cli");
    match get_file(&arg_cli.config) {
        Ok(readed_file) => {
            let config_file = serde_json::from_slice::<config_file::ConfigFile>(&readed_file)
                .expect(&format!("Error parse config file {}", &arg_cli.config));
            println!("{:?}", config_file);

            task_procces::start_procces(&config_file, &arg_cli).await;
        }
        Err(error) => handle_error_system_resources_log(&error),
    };

    Ok(())
}

/*
match deelp::actions::send_petition(&client_deepl, &message).await {
    Ok(a) => {
        let json_strutc:Value = serde_json::from_slice(&a.bytes().await.unwrap()).unwrap();

        println!("{json_strutc}");
        let map = json_strutc.as_object().unwrap();
        for entry in map {
            println!("{} - {}",entry.0,entry.1)
        };
    }
    Err(e) => handle_error_petition_log(&e),
};
 */
