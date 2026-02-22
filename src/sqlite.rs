use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};
use anyhow::Result;
use crate::service;
use serde::de::Error;

pub struct SqliteStore {
    pool: sqlx::SqlitePool,
}

const DEFAULT_DB_PATH: &str = "~/.local/share/gxandria/store.db";

#[derive(Debug, FromRow)]
struct SqliteGame {
    id: String,
    name: String,
    summary: Option<String>,
    storyline: Option<String>,
    genres: Option<String>,
    store_name: String,
    store_id: String,
}

impl From<&service::Game> for SqliteGame {
    fn from(game: &service::Game) -> Self {
        Self {
            id: game.title.clone(), // TODO better id, maybe a hash of name & other fields?
            name: game.title.clone(),
            summary: game.summary.clone(),
            storyline: game.storyline.clone(),
            genres: Some(serde_json::to_string(&game.genres).unwrap_or_else(|_| "[]".to_string())),
            store_name: match &game.store_info {
                service::StoreInfo::Epic { .. } => "epic".to_string(),
                service::StoreInfo::GOG { .. } => "gog".to_string(),
            },
            store_id: match &game.store_info {
                service::StoreInfo::Epic { id } => id.clone(),
                service::StoreInfo::GOG { id } => id.clone(),
            },
        }
    }
}

impl TryFrom<SqliteGame> for service::Game {
    type Error = serde_json::Error;

    fn try_from(game: SqliteGame) -> Result<Self, Self::Error> {
        let maybe_store_info = match game.store_name.as_str() {
            "epic" => Some(service::StoreInfo::Epic { id: game.store_id }),
            "gog" => Some(service::StoreInfo::GOG { id: game.store_id }),
            _ => None,
        };
        if maybe_store_info.is_none() {
            return Err(serde_json::Error::custom(format!("Unknown store name: {}", game.store_name)));
        };

        Ok(Self {
            title: game.name,
            summary: game.summary,
            storyline: game.storyline,
            genres: serde_json::from_str(game.genres.as_deref().unwrap_or("[]"))?,
            store_info: maybe_store_info.unwrap(), // TODO i don't like this unwrap
        })
    }
}

impl SqliteStore {
    pub async fn from_env() -> Result<Self> {
        let path = std::env::var("SQLITE_PATH").unwrap_or_else(|_| DEFAULT_DB_PATH.to_string());

        if let Some(parent) = std::path::Path::new(&path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn_opts = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(conn_opts).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl service::Store for SqliteStore {
    async fn upsert_game(&self, game: &service::Game) -> Result<()> {
        let sqlite_game: SqliteGame = game.into();
        sqlx::query!(
            r#"
            INSERT INTO games (id, name, summary, storyline, genres, store_name, store_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                summary = excluded.summary,
                storyline = excluded.storyline,
                genres = excluded.genres,
                store_name = excluded.store_name,
                store_id = excluded.store_id
            "#,
            sqlite_game.id,
            sqlite_game.name,
            sqlite_game.summary,
            sqlite_game.storyline,
            sqlite_game.genres,
            sqlite_game.store_name,
            sqlite_game.store_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn search_game(&self, title: &str) -> Result<Vec<service::Game>> {
        let like_title = format!("%{}%", title);

        let rows: Vec<SqliteGame> = sqlx::query_as!(
            SqliteGame,
            "SELECT * FROM games WHERE name LIKE ?",
            like_title
        )
        .fetch_all(&self.pool)
        .await?;

        let games = rows.into_iter()
            .filter_map(|row| service::Game::try_from(row).ok())
            .collect();

        Ok(games)
    }
}
