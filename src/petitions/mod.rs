pub mod deelp;

#[derive(Debug)]
pub enum ErrorPetition {
    StatusResponseNotValid(u16),
    ErrorSendRequest(String),
}

pub fn handle_error_petition_log(error: &ErrorPetition) {
    match error {
        ErrorPetition::StatusResponseNotValid(number) => {
            log::error!("Failed response with status: {number}")
        }
        ErrorPetition::ErrorSendRequest(url) => log::error!("Failed send request in {url}"),
    }
}

mod management_response {

    use reqwest::{Response, StatusCode};

    use super::ErrorPetition;

    pub fn validate_status_response(response: &Response) -> Result<(), ErrorPetition> {
        match response.status() {
            StatusCode::OK | StatusCode::ACCEPTED => Ok(()),
            _ => Err(ErrorPetition::StatusResponseNotValid(
                response.status().as_u16(),
            )),
        }
    }
}
