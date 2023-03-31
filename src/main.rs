
use translator_cli::{petitions::{deelp, handle_error_petition_log}, system_resources::{actions::{get_file_to_string}, handle_error_system_resources_log, model::ConfigFile}};

#[tokio::main]
async fn main() {

    match get_file_to_string("./config_file.json") {
        Ok(readed_file) => {
            let config_file = serde_json::from_str::<ConfigFile>(&readed_file).unwrap();
            println!("{:?}",config_file);

            let client_deepl = deelp::actions::build_client_to_deepl(&config_file.api_key_deepl.unwrap());

            let message = deelp::model::Message {
                cod_lang: "EN",
                text: "Hola Mundo",
            };
        
            match deelp::actions::send_petition(&client_deepl, &message).await {
                Ok(a) => {
                    let json_strutc: deelp::model::ResponseBodyDeepl = a.json().await.unwrap();
                    json_strutc
                        .translations
                        .iter()
                        .for_each(|tr| println!("{tr}"))
                }
                Err(e) => handle_error_petition_log(&e),
            };
        },
        Err(error) => handle_error_system_resources_log(&error)
    };


    
}
