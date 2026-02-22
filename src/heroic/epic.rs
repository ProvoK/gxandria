use anyhow::{Context, Result};
use serde::Deserialize;
use tracing::info;

use crate::service::{LauncherGame, StoreName};

#[derive(Debug, Deserialize)]
pub struct EpicLibraryData {
    pub library: Vec<EpicGameData>,
}

#[derive(Debug, Deserialize)]
pub struct EpicGameData {
    pub app_name: String,
    pub title: String,
}

pub struct EpicLibrary {
    base_path: String,
}

impl EpicLibrary {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn list_games(&self) -> Result<Vec<LauncherGame>> {
        let epic_path = format!("{}/store_cache/legendary_library.json", self.base_path);
        info!("Reading Epic Games library from {}", epic_path);
        let file = std::fs::File::open(&epic_path).context("Failed to open legendary_library.json")?;
        let reader = std::io::BufReader::new(file);
        let games: EpicLibraryData = serde_json::from_reader(reader).context("Failed to read legendary_library.json")?;


        let store_games = games.library.into_iter().map(|game| LauncherGame {
            title: game.title,
            app_name: game.app_name,
            store: StoreName::Epic,
        }).collect();

        Ok(store_games)
    }
}
