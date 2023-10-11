use serde::{Deserialize, Serialize};
use crate::models::source::Source;


#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub struct ModDef {
    pub source_id: String,
    pub source: Source,
    pub mod_id: String,
    pub mod_version: String,
    pub loader: Loader,
    pub depends: Vec<String>,
    pub breaks: Vec<String>
}

#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub enum Loader {
    Fabric,
    Quilt,
    Forge,
    NeoForge
}

impl ModDef {
    fn get_download_url(&self) -> String {
        todo!()
    }
}