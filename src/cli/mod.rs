use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

//TODO

#[derive(Parser)]
#[command(name = "Iris-Cli")]
#[command(author = "OnSystem")]
#[command(version = "1.0")]
//#[command(about = "", long_about = None)]
pub struct Cli {
    /// Define level debug to show log info
    #[arg(short, action = ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    pub verbose: u8,
    /// Define path config_file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// Define the language to be translated
    #[arg(short, long)]
    pub language: Option<String>,
    /// Define file to translate
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// Define path to export data translations
    #[arg(short, long)]
    pub export: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Json format
    Json {
        field_translate: Vec<String>,
    },
    /// Sql format
    Sql {
        field_index_translate: Vec<u8>,
    },
    /// Translate a word
    Text { text_translate: Option<String> },
    /// Make Template config
    Template,
}
