use serde::{Serialize, Deserialize};




#[derive(Debug,Serialize,Deserialize)]
pub struct ConfigFile{

    pub api_key_deepl: Option<String>,

}