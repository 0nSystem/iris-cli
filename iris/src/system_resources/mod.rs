/*! The system_resource module allows you to interact with the system file system or actions that are implicit.
 * It allows you to create, read, and check that files exist, such as resolving paths or checking a file.
*/

use color_eyre::{Report, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

/// It allows reading a file, making several verifications to know if
/// it will be able to read it and returning an error if it is not possible to access the information
/// and finally returns it as a string.
/// ```
/// use std::fs::{File,remove_file};
/// use std::path::PathBuf;
/// use iris::system_resources::get_file_to_string;
///
/// let path = PathBuf::from("file.txt");
/// let mut file = File::create(&path).unwrap();
/// let string_read = get_file_to_string(&path).unwrap();
/// remove_file(&path).unwrap();
/// assert_eq!(string_read,"");
///
/// ```
pub fn get_file_to_string(path: &PathBuf) -> Result<String, Report> {
    let files_bytes = read_file(path)?;
    Ok(String::from_utf8(files_bytes)?)
}

/// Create a file if it does not exist and write to it
/// ```
/// use iris::system_resources::create_and_write_file;
/// use iris::system_resources::get_file_to_string;
/// use std::path::PathBuf;
/// use std::fs::{remove_file};
///
/// let text_to_write = "Hello World";
/// let path = PathBuf::from("file.txt");
///
/// create_and_write_file(&path,text_to_write).unwrap();
/// let text_readed_file = get_file_to_string(&path).unwrap();
/// remove_file(&path).unwrap();
///
/// assert_eq!(text_readed_file,text_to_write);
///
/// ```
pub fn create_and_write_file(path: &PathBuf, text: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write(text.as_bytes())
        .map_err(|e| Report::msg(format!("Error write file in: {:?}, error: {}", path, e)))?;
    Ok(())
}

/// It allows reading a file, making several verifications to know if
/// it will be able to read it and returning an error if it is not possible to access the information
/// and finally returns byte vector.
///
/// ```
/// use std::fs::{File,remove_file};
/// use std::path::PathBuf;
/// use iris::system_resources::read_file;
///
/// let path = PathBuf::from("file.txt");
/// let mut file = File::create(&path).unwrap();
/// let string_read = read_file(&path).unwrap();
/// remove_file(&path).unwrap();
///
/// assert_eq!(string_read,"".to_owned().as_bytes());
///
/// ```
pub fn read_file(path: &PathBuf) -> Result<Vec<u8>> {
    exist_file_or_directory(path)?;
    is_file(path)?;
    Ok(fs::read(path)?)
}

/// Determines if the specified path is valid or not, in case of error returning the corresponding information.
fn exist_file_or_directory(path: &PathBuf) -> Result<()> {
    match path.exists() {
        true => Ok(()),
        false => Err(Report::msg(format!(
            "Not exist path: {}",
            path_to_str(path)?
        ))),
    }
}
fn is_file(path: &PathBuf) -> Result<()> {
    match path.is_file() {
        true => Ok(()),
        false => Err(Report::msg(format!("Isnt file: {:?}", path))),
    }
}

/// Allows to parse a path and obtain the equivalence to a string.
///```
///
/// use iris::system_resources::path_to_str;
/// use std::path::PathBuf;
///
/// let text_path = "./";
///
/// let path_buf = PathBuf::from(text_path);
/// let parse_path_buf_to_string = path_to_str(&path_buf).unwrap();
///
/// assert_eq!(parse_path_buf_to_string,text_path)
///
///```

pub fn path_to_str(path: &PathBuf) -> Result<String> {
    match path.to_str() {
        Some(path_parsed_to_string) => Ok(path_parsed_to_string.to_owned()),
        None => Err(Report::msg(format!(
            "Error parse to string path: {:?}",
            path
        ))),
    }
}
