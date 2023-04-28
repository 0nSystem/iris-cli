
use crate::cli::Cli;
use crate::system_resources::actions;
mod template_procces;

pub async fn start_procces<'a>(args_cli: &'a Cli) -> Result<(),TaskError> {
    log::info!("Start procces to api request");

    let output: String = match &args_cli.command {
        crate::cli::Commands::Json { field_translate } => todo!(),
        crate::cli::Commands::Sql {
            field_index_translate,
        } => todo!(),
        crate::cli::Commands::Text { text_translate } => todo!(),
        crate::cli::Commands::Template => template_procces::create_default_template()?,
    };

    if let Some(file_export) = &args_cli.export {
        actions::create_and_write_file(file_export, &output)
    } else {
        println!("{output}")
    }

    log::info!("Finish procces to api request");

    Ok(())
}
//TODO change params

//TODO move cant access other modules
 pub enum TaskError {

    ErrorRequireField(String),
    ErrorCreateTemplate
}
