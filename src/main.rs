use clap::Parser;
use color_eyre::{eyre::Result, Report};
use iris_cli::utils::logger::config_logger;
use iris_cli::{cli, task_procces};

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let arg_cli = cli::Cli::parse();

    config_logger(arg_cli.verbose, env_logger::Target::Stdout).expect("Error config logger");

    log::info!("Starting translation-cli");

    task_procces::start_procces(&arg_cli).await?;
    Ok(())
}
