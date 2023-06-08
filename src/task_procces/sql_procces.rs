use std::collections::HashMap;

use color_eyre::Report;

use crate::{cli::ModeSql, system_resources::model::config_file::ConfigFile};

pub fn config_and_run_json_command<'a>(
    indexs: &'a Vec<String>,
    mode: &'a ModeSql,
    language: &'a String,
    config_file: &'a ConfigFile,
) -> Result<HashMap<String, String>, Report> {
    Err(Report::msg("todo"))
}
