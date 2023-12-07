use crate::db::{config::read_config, scan::scan_and_insert};

use super::State;

#[tauri::command]
#[specta::specta]
pub async fn scan_start(state: State<'_>) -> Result<(), String> {
    let config = read_config(&state.db).await.map_err(|e| e.to_string())?;
    scan_and_insert(&state.db, state.client.clone(), config.scan_dir)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
