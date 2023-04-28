pub mod model;

pub mod management_errors {
    use log::error;
    pub enum ErrorSystemResources {
        FailedResolvedRoute(String),
        IsNotFile(String),
        IsNotDirectory(String),
        FileNotExist(String),
        CantReadFile(String),
        CantParseToString,
    }

    pub fn handle_error_system_resources_log(error: &ErrorSystemResources) {
        match error {
            ErrorSystemResources::FailedResolvedRoute(path) => {
                error!("The route cannot be resolved: {path}")
            }
            ErrorSystemResources::IsNotFile(path) => {
                error!("The specified path {path} is not a file")
            }
            ErrorSystemResources::IsNotDirectory(path) => {
                error!("The specified path {path} is not a directory")
            }
            ErrorSystemResources::FileNotExist(path) => {
                error!("File or directory {path} does not exist")
            }
            ErrorSystemResources::CantReadFile(path) => error!("Cannot read file {path}"),
            ErrorSystemResources::CantParseToString => error!("Information cannot be parsed"),
        }
    }
}

pub mod actions {
    use super::management_errors::ErrorSystemResources;
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    pub fn get_file_to_string<'a>(path: &'a str) -> Result<String, ErrorSystemResources> {
        let files_bytes = get_file(path)?;
        match String::from_utf8(files_bytes) {
            Ok(string_readed) => Ok(string_readed),
            Err(_) => Err(ErrorSystemResources::CantParseToString),
        }
    }
    pub fn get_file<'a>(path: &'a str) -> Result<Vec<u8>, ErrorSystemResources> {
        match fs::canonicalize(path) {
            Ok(path_buf) => read_file(&path_buf),
            Err(_) => Err(ErrorSystemResources::FailedResolvedRoute(path.to_string())),
        }
    }

    //TODO result
    pub fn create_and_write_file<'a>(path: &'a PathBuf, text: &'a str) {
        let mut file = File::create(&path).expect("Error create new file");

        file.write(text.as_bytes());
    }

    fn read_file<'a>(path: &'a PathBuf) -> Result<Vec<u8>, ErrorSystemResources> {
        exist_file_or_directory(path)?;
        is_file(path)?;
        match fs::read(path) {
            Ok(result_read) => Ok(result_read),
            Err(_) => Err(ErrorSystemResources::CantReadFile(path_to_str(path))),
        }
    }

    fn exist_file_or_directory<'a>(path: &'a PathBuf) -> Result<(), ErrorSystemResources> {
        match path.exists() {
            true => Ok(()),
            false => Err(ErrorSystemResources::FileNotExist(path_to_str(path))),
        }
    }
    fn is_file<'a>(path: &'a PathBuf) -> Result<(), ErrorSystemResources> {
        match path.is_file() {
            true => Ok(()),
            false => Err(ErrorSystemResources::IsNotFile(path_to_str(path))),
        }
    }

    pub fn path_to_str<'a>(path: &'a PathBuf) -> String {
        path.to_str()
            .expect("Error parse path to string")
            .to_string()
    }
}
