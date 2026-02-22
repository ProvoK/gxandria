use anyhow::{Context, Result};
use serde::Deserialize;
use tracing::info;

use crate::service::{LauncherGame, StoreName};

#[derive(Debug, Deserialize)]
pub struct GOGLibraryData {
    pub games: Vec<GOGGameData>,
}

#[derive(Debug, Deserialize)]
pub struct GOGGameData {
    pub app_name: String,
    pub title: String,
}

pub struct GOGLibrary {
    base_path: String,
}

impl GOGLibrary {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn list_games(&self) -> Result<Vec<LauncherGame>> {
        let epic_path = format!("{}/store_cache/gog_library.json", self.base_path);
        info!("Reading GOG Games library from {}", epic_path);
        let file = std::fs::File::open(&epic_path).context("Failed to open gog_library.json")?;
        let reader = std::io::BufReader::new(file);
        let games: GOGLibraryData = serde_json::from_reader(reader).context("Failed to read legendary_library.json")?;


        let store_games = games.games.into_iter().map(|game| LauncherGame {
            title: game.title,
            app_name: game.app_name,
            store: StoreName::GOG,
        }).collect();

        Ok(store_games)
    }
}
