use anyhow::{Context, Result};
use serde_json::{json, Value};
use tokio::fs;
use tracing::info;

use crate::{heroic::{epic::EpicLibrary, gog::GOGLibrary}, service::{Game, Launcher, LauncherGame, StoreInfo, StoreName}};

pub struct Store {
    epic: EpicLibrary,
    gog: GOGLibrary,
}

const DEFAULT_HEROIC_PATH: &str = "~/.config/heroic";

impl Store {
    pub fn new() -> Self {
        Self {
            epic: EpicLibrary::new(shellexpand::tilde(DEFAULT_HEROIC_PATH).to_string()),
            gog: GOGLibrary::new(shellexpand::tilde(DEFAULT_HEROIC_PATH).to_string()),
        }
    }

}

impl Launcher for Store {

    async fn list_games(&self) -> Result<Vec<LauncherGame>> {
        let epic_games = self.epic.list_games()?;
        let gog_games = self.gog.list_games()?;
        Ok([epic_games, gog_games].concat())
    }

    async fn update_custom_categories(&self, all_games: Vec<Game>) -> Result<()> {
        let config_path = format!("{}/store/config.json", shellexpand::tilde(DEFAULT_HEROIC_PATH));
        let backup_path = format!("{}.bak", config_path);
        if !std::path::Path::new(&backup_path).exists() {
            info!("Creating backup of config.json at {}", backup_path);
            std::fs::copy(&config_path, &backup_path).context("Failed to create backup of config.json")?;
        }
        
        // TODO: maybe get existing custom categories from backup file and merge with new ones, to
        // avoid losing user custom categories that are not related to genres

        let custom_categories = all_games.into_iter().fold(std::collections::HashMap::new(), |mut acc, game| {
            for genre in game.genres {
                let final_appname = match game.store_info {
                    StoreInfo::Epic { ref id } => format!("{}_legendary", id),
                    StoreInfo::GOG { ref id } => format!("{}_gog", id),
                };
                acc.entry(genre).or_insert_with(Vec::new).push(final_appname);
            }
            acc
        });

        info!("Generated custom categories: {:#?}", custom_categories);

        let config_data = fs::read_to_string(&config_path).await.context("Failed to read config.json")?;
        let mut root: Value = serde_json::from_str(&config_data).context("Failed to parse config.json")?;

        let categories = root.as_object_mut()
            .expect("root should be an object")
            .entry("games")
            .or_insert(json!({}))
            .as_object_mut()
            .expect("games should be an object")
            .entry("customCategories")
            .or_insert(json!({}));

        *categories = json!(custom_categories);

        let updated = serde_json::to_string_pretty(&root).context("Failed to serialize updated config.json")?;
        fs::write(&config_path, updated).await.context("Failed to write updated config.json")?;

        Ok(())
    }
}
