use anyhow::Result;
use async_trait::async_trait;

use super::GameMetadata;


#[async_trait]
pub trait Client {
    async fn search_game(&self, title: &str) -> Result<Vec<GameMetadata>>;
}
