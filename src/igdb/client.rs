use crate::service;

use super::types::{ApiGame, ApiGenericAttribute};

use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Url;
use serde::{Deserialize, de::DeserializeOwned};
use tracing::{debug, error, info};
use moka::future::Cache;

const IGDB_BASE_URL: &str = "https://api.igdb.com/v4";
const TWITCH_OAUTH2_URL: &str = "https://id.twitch.tv/oauth2/token";

pub struct ApiClient {
    client_id: String,
    client_token: String,
    http_client: reqwest::Client,
    cache: Cache<String, String>,
}

#[derive(Deserialize)]
struct OauthResponse {
    pub access_token: String,
    //pub token_type: String,
    //pub expires_in: i64,
}

impl ApiClient {
    pub async fn from_env() -> Result<Self> {
        let id = std::env::var("IGDB_CLIENT_ID")
            .context("Env var IGDB_CLIENT_ID must be set")?.trim().to_string();
        let secret = std::env::var("IGDB_CLIENT_SECRET")
            .context("Env var IGDB_CLIENT_SECRET must be set")?.trim().to_string();
        
        let token = ApiClient::obtain_token(id.as_str(), secret.as_str()).await?;

        Ok(Self {
            client_id: id.clone(),
            client_token: token,
            http_client: reqwest::Client::new(),
            cache: Cache::new(100_000),
        })
    }

    async fn obtain_token(id: &str, secret: &str) -> Result<String> {
        let mut oauth_url = Url::parse(TWITCH_OAUTH2_URL)?;
        let client = reqwest::Client::new();
        let query = format!(
            "client_id={}&client_secret={}&grant_type=client_credentials",
            id, secret,
        );
        oauth_url.set_query(Some(query.as_str()));
        tracing::debug!("getting token");
        let res: OauthResponse = client
            .post(oauth_url)
            .send()
            .await?
            .json()
            .await?;
        debug!("token retrieved");
        Ok(res.access_token.clone())
    }

    // query should be in APICalypse, see: https://api-docs.igdb.com/?shell#apicalypse-1
    async fn get_vec<T: DeserializeOwned>(
        &self,
        resource: &str,
        query: Option<String>,
    ) -> Result<Vec<T>> {
        let api_url = Url::parse(format!("{}/{}", IGDB_BASE_URL, resource).as_str())?;
        let query_body = query.unwrap_or("fields *; limit 500;".to_string());

        let response = self
            .http_client
            .post(api_url.clone())
            .header("Client-ID", self.client_id.clone())
            .bearer_auth(self.client_token.clone())
            .header("Accept", "application/json")
            .body(query_body.clone())
            .send()
            .await
            .with_context(|| format!("Failed to reach IGDB at {}", api_url))?;

        let status = response.status();
        if !status.is_success() {
            let err_body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %err_body, "IGDB API Error");
            anyhow::bail!("IGDB returned error status {}: {}", status, err_body);
        }

        let raw_text = response
            .text()
            .await
            .context("Failed to read response body as text")?;

        let parsed = serde_json::from_str::<Vec<T>>(&raw_text)
            .with_context(|| {
                error!(raw_body = %raw_text, "Failed to deserialize IGDB response");
                format!("Failed to parse JSON for resource '{}'. Raw body: {}", resource, raw_text)
            })?;

        Ok(parsed)
    }

    async fn fetch_genres(&self) -> Result<()> {
        if let Some(_) = self.cache.get("genres").await {
            return Ok(());
        }

        info!("Fetching genres from IGDB API");

        let genres = self.get_vec::<ApiGenericAttribute>("genres", None).await?;
        for genre in &genres {
            let key = format!("genre:{}", genre.id);
            let value = serde_json::to_string(&genre)?;
            self.cache.insert(key, value).await;
        }
        self.cache.insert("genres".to_string(), "true".to_string()).await;

        info!("Genres fetched and cached successfully");

        Ok(())
    }

    async fn get_genre(&self, genre_id: u64) -> Result<ApiGenericAttribute> {
        let key = format!("genre:{}", genre_id);
        if let None = self.cache.get("genres").await {
            info!("Genres not found in cache, fetching from API");
            self.fetch_genres().await?;
        }

        if let Some(value) = self.cache.get(&key).await {
            let genre: ApiGenericAttribute = serde_json::from_str(&value)?;
            Ok(genre)
        } else {
            anyhow::bail!("Genre with ID {} not found in cache", genre_id);
        }
    }

    async fn get_game(&self, title: &str) -> Result<Vec<ApiGame>> {
        let query = format!("fields *; search \"{}\"; limit 50;", title);
        self.get_vec::<ApiGame>("games", Some(query)).await
    }
}

#[async_trait]
impl service::Client for ApiClient {

    async fn search_game(&self, title: &str) -> Result<Vec<service::GameMetadata>> {
        let mut games = vec![];

        let api_games = self.get_game(title).await?;

        for api_game in api_games {
            let mut genres = vec![];

            for genre_id in api_game.genres {
                match self.get_genre(genre_id).await {
                    Ok(genre) => genres.push(genre.name),
                    Err(e) => error!(%e, genre_id, "Failed to get genre for game '{}'", api_game.name),
                }
            }

            games.push(service::GameMetadata {
                title: api_game.name,
                summary: api_game.summary,
                storyline: api_game.storyline,
                genres,
            });
        }

        Ok(games)
    }
}
