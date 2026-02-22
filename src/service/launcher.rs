use anyhow::Result;

use super::{Game, StoreName};

#[derive(Debug, Clone)]
pub struct LauncherGame {
    pub title: String,
    pub app_name: String,
    pub store: StoreName,
}

pub trait Launcher {
    async fn list_games(&self) -> Result<Vec<LauncherGame>>;
    async fn update_custom_categories(&self, all_games: Vec<Game>) -> Result<()>;
}
