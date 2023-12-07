use crate::{
    db::config::{read_config, update_config},
    interface::config::Config,
};

use super::State;

#[tauri::command]
#[specta::specta]
pub async fn config_read(state: State<'_>) -> Result<Config, String> {
    read_config(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn config_write(state: State<'_>, config: Config) -> Result<(), String> {
    update_config(&state.db, config)
        .await
        .map_err(|e| e.to_string())
}
