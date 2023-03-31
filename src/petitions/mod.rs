pub mod deelp;

#[derive(Debug)]
pub enum ErrorPetition {
    StatusResponseNotValid(u16),
}

pub fn handle_error_petition_log(error: &ErrorPetition) {
    match error {
        ErrorPetition::StatusResponseNotValid(number) => println!("{number}"),
    }
}
