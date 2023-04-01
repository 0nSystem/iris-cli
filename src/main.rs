use color_eyre::{eyre::Result, Report};
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
    config_logger(0, env_logger::Target::Stdout).expect("Error config logger");


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
                    let json_strutc: deelp::model::ResponseBodyDeepl = a.json().await.unwrap();
                    json_strutc
                        .translations
                        .iter()
                        .for_each(|tr| println!("{:?}",tr))
                }
                Err(e) => handle_error_petition_log(&e),
            };
        }
        Err(error) => handle_error_system_resources_log(&error),
    };

    Ok(())
}
