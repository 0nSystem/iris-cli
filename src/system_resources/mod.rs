pub mod model;

pub mod management_errors {
    pub enum ErrorSystemResources {
        FailedResolvedRoute(String),
        IsNotFile(String),
        IsNotDirectory(String),
        FileNotExist(String),
        CantReadFile(String),
        CantWriteFile(String),
        CantParseToString,
    }

    pub fn handle_error_system_resources_log(error: &ErrorSystemResources) -> String {
        match error {
            ErrorSystemResources::FailedResolvedRoute(path) => {
                format!("The route cannot be resolved {path}")
            }
            ErrorSystemResources::IsNotFile(path) => {
                format!("The specified path {path} is not a file")
            }
            ErrorSystemResources::IsNotDirectory(path) => {
                format!("The specified path {path} is not a directory")
            }
            ErrorSystemResources::FileNotExist(path) => {
                format!("File or directory {path} does not exist")
            }
            ErrorSystemResources::CantReadFile(path) => format!("Cannot read file {path}"),
            ErrorSystemResources::CantParseToString => format!("Information cannot be parsed"),
            ErrorSystemResources::CantWriteFile(path) => format!("Cant write file: {path}"),
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

    pub fn get_file_to_string<'a>(path: &'a PathBuf) -> Result<String, ErrorSystemResources> {
        let files_bytes = read_file(path)?;
        match String::from_utf8(files_bytes) {
            Ok(string_readed) => Ok(string_readed),
            Err(_) => Err(ErrorSystemResources::CantParseToString),
        }
    }

    //TODO result
    pub fn create_and_write_file<'a>(
        path: &'a PathBuf,
        text: &'a str,
    ) -> Result<(), ErrorSystemResources> {
        let mut file = File::create(&path).expect("Error create new file");

        if let Err(_) = file.write(text.as_bytes()) {
            return Err(ErrorSystemResources::CantWriteFile(path_to_str(path)?));
        }

        Ok(())
    }

    pub fn read_file<'a>(path: &'a PathBuf) -> Result<Vec<u8>, ErrorSystemResources> {
        exist_file_or_directory(path)?;
        is_file(path)?;
        match fs::read(path) {
            Ok(result_read) => Ok(result_read),
            Err(_) => Err(ErrorSystemResources::CantReadFile(path_to_str(path)?)),
        }
    }

    fn exist_file_or_directory<'a>(path: &'a PathBuf) -> Result<(), ErrorSystemResources> {
        match path.exists() {
            true => Ok(()),
            false => Err(ErrorSystemResources::FileNotExist(path_to_str(path)?)),
        }
    }
    fn is_file<'a>(path: &'a PathBuf) -> Result<(), ErrorSystemResources> {
        match path.is_file() {
            true => Ok(()),
            false => Err(ErrorSystemResources::IsNotFile(path_to_str(path)?)),
        }
    }

    pub fn path_to_str<'a>(path: &'a PathBuf) -> Result<String, ErrorSystemResources> {
        match path.to_str() {
            Some(path_parsed_to_string) => Ok(path_parsed_to_string.to_owned()),
            None => Err(ErrorSystemResources::FailedResolvedRoute(
                "in parsed".to_owned(),
            )),
        }
    }
}
