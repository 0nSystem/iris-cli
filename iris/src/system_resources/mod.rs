pub mod actions {
    use color_eyre::{Report, Result};

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
    pub fn create_and_write_file(path: &PathBuf, text: &str) -> Result<()> {
        let mut file = File::create(path)?;
        file.write(text.as_bytes())
            .map_err(|e| Report::msg(format!("Error write file in: {:?}, error: {}", path, e)))?;
        Ok(())
    }

    pub fn read_file(path: &PathBuf) -> Result<Vec<u8>> {
        exist_file_or_directory(path)?;
        is_file(path)?;
        Ok(fs::read(path)?)
    }

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

    pub fn path_to_str(path: &PathBuf) -> Result<String> {
        match path.to_str() {
            Some(path_parsed_to_string) => Ok(path_parsed_to_string.to_owned()),
            None => Err(Report::msg(format!(
                "Error parse to string path: {:?}",
                path
            ))),
        }
    }
}
