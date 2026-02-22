use anyhow::Result;
use async_trait::async_trait;
use super::Game;

#[async_trait]
pub trait Store {
    async fn search_game(&self, title: &str) -> Result<Vec<Game>>;
    async fn upsert_game(&self, game: &Game) -> Result<()>;
}
