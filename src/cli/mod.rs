use clap::{ArgAction, Parser, Subcommand};

//TODO

#[derive(Parser)]
#[command(name = "Translator-Cli")]
#[command(author = "OnSystem")]
#[command(version = "1.0")]
//#[command(about = "", long_about = None)]
pub struct Cli {
    /// Define level debug to show log info
    #[arg(short, action = ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    pub verbose: u8,
    /// Define path config_file
    #[arg(short,long)]
    pub config: String,
    /// Define the language to be translated
    #[arg(short, long)]
    pub language: String,
    /// Define file to translate
    #[arg(short, long)]
    pub file: Option<String>,
    /// Define path to export data translations
    #[arg(short, long)]
    pub export: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Json format
    Json {
        #[arg(short, long)]
        field_transalte: Vec<String>,
    },
    /// Sql format
    Sql {
        #[arg(short, long)]
        field_index_translate: Vec<u8>,
    },
    /// Translate a word
    Text{
        text_transalate: String
    }
}
