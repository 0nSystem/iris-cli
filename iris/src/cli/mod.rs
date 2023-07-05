/*!
 * All information concerning the command line.
 */

use clap::{ArgAction, Parser, Subcommand};
use std::{fmt::Display, path::PathBuf};

/// It is the variety of command line commands allowed to access all the functionalities of the application.
#[derive(Parser)]
#[command(author = "OnSystem", version)]
//#[command(about = "", long_about = None)]
pub struct Cli {
    /// Define level debug to show log info.
    #[arg(short, action = ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    pub verbose: u8,
    /// Define path config_file.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// Define the language to be translated.
    #[arg(short, long)]
    pub language: Option<String>,
    /// Define file to translate.
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// Define path to export data translations.
    #[arg(short, long)]
    pub export: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Json format
    Json {
        /// Requires the use of the json path expression, defined in the library: [`jsonpath_lib`].
        field_translate: Vec<String>,
    },
    /// Sql format
    Sql {
        /// Field 0 also counts in principle.
        #[arg(short, long)]
        field_index: String,
        #[command(subcommand)]
        mode: ModeSql,
    },
    /// Translate a text or word.
    Text { text_translate: Option<String> },
    /// Make Template config.
    Template,
}

#[derive(Subcommand, PartialEq, Eq)]

pub enum ModeSql {
    Insert,
    Update,
}

impl Display for ModeSql {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeSql::Insert => write!(f, "Insert"),
            ModeSql::Update => write!(f, "Update"),
        }
    }
}
