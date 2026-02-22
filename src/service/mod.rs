mod client;
mod service;
mod store;
mod launcher;

pub use client::*;
use serde::Serialize;
pub use service::*;
pub use store::*;
pub use launcher::*;

#[derive(Debug, Clone, Serialize)]
pub struct Game {
    pub title: String,
    pub summary: Option<String>,
    pub storyline: Option<String>,
    pub genres: Vec<String>,
    pub store_info: StoreInfo,
}

#[derive(Debug, Clone, Serialize)]
pub enum StoreName {
    Epic,
    GOG,
}

impl From<StoreName> for String {
    fn from(store_name: StoreName) -> Self {
        match store_name {
            StoreName::Epic => "epic".to_string(),
            StoreName::GOG => "gog".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GameMetadata {
    pub title: String,
    pub summary: Option<String>,
    pub storyline: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum StoreInfo {
    Epic { id: String },
    GOG { id: String },
    // TODO add more stores if needed
}
