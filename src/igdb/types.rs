use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGame {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub url: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
    pub checksum: String,

    pub summary: Option<String>,
    pub storyline: Option<String>,
    pub rating: Option<f64>,
    pub rating_count: Option<u64>,
    pub total_rating: Option<f64>,
    pub total_rating_count: Option<u64>,
    pub hypes: Option<u64>,
    pub game_type: Option<u64>,
    pub first_release_date: Option<u64>,
    pub cover: Option<u64>,

    #[serde(default)]
    pub external_games: Vec<u64>,
    #[serde(default)]
    pub game_engines: Vec<u64>,
    #[serde(default)]
    pub involved_companies: Vec<u64>,
    #[serde(default)]
    pub platforms: Vec<u64>,
    #[serde(default)]
    pub release_dates: Vec<u64>,
    #[serde(default)]
    pub websites: Vec<u64>,
    #[serde(default)]
    pub age_ratings: Vec<u64>,
    #[serde(default)]
    pub alternative_names: Vec<u64>,
    #[serde(default)]
    pub artworks: Vec<u64>,
    #[serde(default)]
    pub game_modes: Vec<u64>,
    #[serde(default)]
    pub genres: Vec<u64>,
    #[serde(default)]
    pub keywords: Vec<u64>,
    #[serde(default)]
    pub player_perspectives: Vec<u64>,
    #[serde(default)]
    pub screenshots: Vec<u64>,
    #[serde(default)]
    pub similar_games: Vec<u64>,
    #[serde(default)]
    pub tags: Vec<u64>,
    #[serde(default)]
    pub themes: Vec<u64>,
    #[serde(default)]
    pub videos: Vec<u64>,
    #[serde(default)]
    pub language_supports: Vec<u64>,
}

// Same model for:
// - game_modes
// - genres
// - player_perspectives
// - themes
// - keywords
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGenericAttribute {
    pub id: u64,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGameType {
    pub id: u64,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub name: String,
    pub checksum: String,
}
