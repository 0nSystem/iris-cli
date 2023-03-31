
pub mod model;


pub enum ErrorSystemResources {
    FailedResolvedRoute,
    IsNotFile,
    IsNotDirectory,
    NotExist,
    CantReadFile,
    CantParseToString,
}


pub fn handle_error_system_resources_log(error: &ErrorSystemResources){
    match error {
        ErrorSystemResources::FailedResolvedRoute => todo!(),
        ErrorSystemResources::IsNotFile => todo!(),
        ErrorSystemResources::IsNotDirectory => todo!(),
        ErrorSystemResources::NotExist => todo!(),
        ErrorSystemResources::CantReadFile => todo!(),
        ErrorSystemResources::CantParseToString => todo!(),
    }
}


pub mod actions {
    use std::{fs, path::PathBuf};
    use super::ErrorSystemResources;

    pub fn get_file_to_string<'a>(path: &'a str) -> Result<String,ErrorSystemResources>{
        let files_bytes = get_file(path)?;
        match String::from_utf8(files_bytes) {
            Ok(string_readed) => Ok(string_readed),
            Err(_) => Err(ErrorSystemResources::CantParseToString),
        }
    }
    pub fn get_file<'a>(path: &'a str) -> Result<Vec<u8>,ErrorSystemResources>{
        match fs::canonicalize(path) {
            Ok(path_buf) => read_file(&path_buf),
            Err(_) => Err(ErrorSystemResources::FailedResolvedRoute),
        }
    }
    
    fn read_file<'a>(path: &'a PathBuf) -> Result<Vec<u8>,ErrorSystemResources>{
        exist_file_or_directory(path)?;
        is_file(path)?;
        match fs::read(path) {
            Ok(result_read) => Ok(result_read),
            Err(_) => Err(ErrorSystemResources::CantReadFile),
        } 
    }
    
    fn exist_file_or_directory<'a>(path: &'a PathBuf) -> Result<(),ErrorSystemResources> {
        match path.exists() {
            true => Ok(()),
            false => Err(ErrorSystemResources::NotExist),
        }
    }
    fn is_file<'a>(path: &'a PathBuf) -> Result<(),ErrorSystemResources>{
        match path.is_file() {
            true => Ok(()),
            false => Err(ErrorSystemResources::IsNotFile),
        }
    }
}