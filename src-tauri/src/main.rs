// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::Arc};

use commands::Ctx;
use db::config::setup_config;
use entity::models;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tracing::info;

mod commands;
mod db;
mod interface;

#[cfg(not(debug_assertions))]
use tokio::fs::create_dir_all;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = create_connection().await;
    run_migration(&pool).await;
    let tray = create_tray();

    if (setup_config(&pool).await).is_ok() {
        info!("Created config");
    } else {
        info!("Config already exists");
    }

    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        specta::collect_types![
            commands::config::config_read,
            commands::config::config_write,
            commands::scan::scan_start
        ],
        "../src/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .manage(Ctx {
            db: pool,
            client: erogamescape_api::Client::default(),
        })
        .invoke_handler(tauri::generate_handler![
            commands::config::config_read,
            commands::config::config_write,
            commands::scan::scan_start
        ])
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn create_connection() -> Arc<DatabaseConnection> {
    #[cfg(debug_assertions)]
    let db_path = {
        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_manifest_dir.join("..").join("data").join("db.sqlite")
    };

    #[cfg(not(debug_assertions))]
    let db_path = {
        let db_dir = dirs::data_local_dir()
            .expect("Could not find config directory")
            .join("hgame-launcher");
        create_dir_all(&db_dir)
            .await
            .expect("Could not create config directory");

        let db_path = db_dir.join("db.sqlite");
        info!("db path: {}", db_path.to_string_lossy());
        db_path
    };

    info!("Database path: {}", db_path.to_string_lossy());

    let opt = ConnectOptions::new(db_path.to_str().unwrap());
    let db = Database::connect(opt).await.unwrap();

    Arc::new(db)
}

async fn run_migration(db: &DatabaseConnection) {
    info!("Running migration");
    migration::Migrator::up(db, None).await.unwrap();
}

fn create_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    SystemTray::new().with_menu(tray_menu)
}

