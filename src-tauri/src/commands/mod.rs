use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub mod config;
pub mod scan;

pub struct Ctx {
    pub db: Arc<DatabaseConnection>,
    pub client: erogamescape_api::Client,
}

type State<'a> = tauri::State<'a, Ctx>;
