use anyhow::Result;
use tracing::{debug, info};
use crate::service::{Launcher, StoreInfo, StoreName};

use super::{store::Store, client::Client, Game, LauncherGame};

pub struct ServiceImpl<S: Store, C: Client, L: Launcher> {
    store: S,
    client: C,
    launcher: L,
}


impl <S: Store, C: Client, L: Launcher> ServiceImpl<S, C, L> {
    pub fn new(store: S, client: C, launcher: L) -> Self {
        Self { store, client, launcher }
    }

    fn sanitize_title(&self, title: &str) -> String {
        let title = title.to_lowercase();

        // remove "- XXXX edition" and similar patterns, which are common in game titles and can
        // cause issues while searching
        let re = regex::Regex::new(r"(?i)\s*[-:]\s*.*edition").unwrap();
        let title = re.replace_all(&title, "");

        // remove trademarks and other tedious characters that can cause issues while searching
        let unwanted_chars = vec!['®', '™', '©', '.'];
        let title = title.replace(|c: char| unwanted_chars.contains(&c), "");
        title
    }

    async fn search_game(&self, game: &LauncherGame) -> Result<Game> {
        // First check the local store for cached results
        let sanitized_title = self.sanitize_title(&game.title);
        let mut games = self.store.search_game(&sanitized_title).await?;
        if games.is_empty() {
            let games_meta = self.client.search_game(&sanitized_title).await?;

            let sorted_games = games_meta.into_iter().map(|game| {
                let distance = strsim::levenshtein(&sanitized_title, game.title.to_lowercase().as_str());
                (game, distance)
            }).collect::<Vec<_>>();
            let Some((closest_game, _)) = sorted_games.into_iter().min_by_key(|(_, distance)| *distance) else {
                return Err(anyhow::anyhow!("No game found for title: {} ({})", sanitized_title, game.title));
            };


            let g = Game {
                title: closest_game.title.clone(),
                summary: closest_game.summary.clone(),
                storyline: closest_game.storyline.clone(),
                genres: closest_game.genres.clone(),
                store_info: match &game.store {
                    StoreName::Epic => StoreInfo::Epic { id: game.app_name.clone() },
                    StoreName::GOG => StoreInfo::GOG { id: game.app_name.clone() },
                },
            };

            self.store.upsert_game(&g).await?;

            games = vec![g];
        }

        // sort games by levenshtein distance to the title, get the first one
        let games = games.into_iter().map(|game| {
            let distance = strsim::levenshtein(&sanitized_title.to_lowercase(), game.title.to_lowercase().as_str());
            (game, distance)
        }).collect::<Vec<_>>();
        let Some((game, _)) = games.into_iter().min_by_key(|(_, distance)| *distance) else {
            return Err(anyhow::anyhow!("No game found for title: {} ({})", sanitized_title, game.title));
        };

        Ok(game)
    }

    pub async fn make_custom_categories(&self) -> Result<()> {
        let owned_games = self.launcher.list_games().await?;
        let mut count_found = 0;
        let mut count_not_found = 0;
        debug!("Found {} owned games in launcher", owned_games.len());
        let mut games = Vec::new();
        for (i, game) in owned_games.clone().into_iter().enumerate() {
            // TODO: change with "progress" trait to generalize progress display
            if i % 10 == 0 {
                info!("Processing game {}/{}", i + 1, owned_games.len());
            }

            let search_result = self.search_game(&game).await;
            if let Ok(found_game) = &search_result {
                games.push(found_game.clone());
                count_found += 1;
            } else if let Err(e) = &search_result {
                let err_msg = format!("{e}");
                info!("Game '{}' not found in API: {}", game.title, err_msg);
                count_not_found += 1;
            }
        }
        info!("Finished processing owned games. Found: {}, Not found: {}", count_found, count_not_found);

        self.launcher.update_custom_categories(games.clone()).await?;
        info!("Custom categories updated in launcher");

        Ok(())
    }
}
