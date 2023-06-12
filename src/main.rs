use clap::Parser;
use color_eyre::{eyre::Result, Report};
use iris_cli::utils::logger::config_logger;
use iris_cli::{cli, task_procces};

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    //let arg_cli = cli::Cli::parse();
    let arg_cli = cli::Cli {
        command: cli::Commands::Sql {
            field_index: "0,1".to_string(),
            mode: cli::ModeSql::Insert,
        },
        config: Some("./default_config_file.json".parse()?),
        export: None,
        verbose: 0,
        language: Some("ES".to_owned()),
        file: Some("./test_sql.sql".parse()?),
    };

    config_logger(arg_cli.verbose, env_logger::Target::Stdout).expect("Error config logger");

    log::info!("Starting translation-cli");

    task_procces::start_procces(&arg_cli).await?;
    Ok(())
}
