mod igdb;
mod sqlite;
mod service;
mod heroic;

use anyhow::Result;
use igdb::ApiClient;

use tracing_subscriber;


#[tokio::main]
async fn main() -> Result<()>  {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let svc = service::ServiceImpl::new(
        sqlite::SqliteStore::from_env().await?,
        ApiClient::from_env().await?,
        heroic::Store::new(),
    );

    svc.make_custom_categories().await
}
