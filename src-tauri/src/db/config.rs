use anyhow::Result;
use sea_orm::DatabaseConnection;

use crate::interface::config::Config;

pub async fn read_config(pool: &DatabaseConnection) -> Result<Config> {
    struct ConfigRow {
        value: String,
    }
    entity::models::game;
    let res = query_as!(ConfigRow, "SELECT value FROM config WHERE id=1")
        .fetch_one(pool)
        .await?;

    let config: Config = serde_json::from_str(&res.value)?;

    Ok(config)
}

pub async fn update_config(pool: &sqlx::SqlitePool, config: Config) -> Result<()> {
    let config_str = serde_json::to_string(&config)?;
    query!("UPDATE config SET value=? WHERE id=1", config_str)
        .fetch_one(pool)
        .await?;

    Ok(())
}

pub async fn setup_config(pool: &sqlx::SqlitePool) -> Result<()> {
    let config_str = serde_json::to_string(&Config::default())?;
    query!("INSERT INTO config (id, value) VALUES (1, ?)", config_str)
        .execute(pool)
        .await?;

    Ok(())
}
