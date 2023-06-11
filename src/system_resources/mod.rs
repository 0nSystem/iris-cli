pub mod model;

pub mod actions {
    use color_eyre::Report;

    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    pub fn get_file_to_string(path: &PathBuf) -> Result<String, Report> {
        let files_bytes = read_file(path)?;
        Ok(String::from_utf8(files_bytes)?)
    }

    //TODO result
    pub fn create_and_write_file<'a>(path: &'a PathBuf, text: &'a str) -> Result<(), Report> {
        let mut file = File::create(path)?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }

    pub fn read_file(path: &PathBuf) -> Result<Vec<u8>, Report> {
        exist_file_or_directory(path)?;
        is_file(path)?;
        Ok(fs::read(path)?)
    }

    fn exist_file_or_directory(path: &PathBuf) -> Result<(), Report> {
        match path.exists() {
            true => Ok(()),
            false => Err(Report::msg(format!("This path not exist: {:?}", path))),
        }
    }
    fn is_file(path: &PathBuf) -> Result<(), Report> {
        match path.is_file() {
            true => Ok(()),
            false => Err(Report::msg(format!("This isn`t file: {:?}", path))),
        }
    }

    pub fn path_to_str(path: &PathBuf) -> Result<String, Report> {
        match path.to_str() {
            Some(path_parsed_to_string) => Ok(path_parsed_to_string.to_owned()),
            None => Err(Report::msg(format!("This isn`t file: {:?}", path))),
        }
    }
}
