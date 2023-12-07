use anyhow::Context;
use entity::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use tracing::{debug, info, warn};

#[derive(Serialize, Deserialize, Debug)]
struct EsGame {
    id: i64,
    gamename: String,
    brandname: i64,
    gyutto_id: Option<i64>,
    dmm: Option<String>,
    dmm_genre: Option<String>,
    dmm_genre_2: Option<String>,
    dlsite_id: Option<String>,
    dlsite_domain: Option<String>,
    digiket: Option<String>,
    comike: Option<i64>,
}

#[tracing::instrument(err, skip_all)]
pub async fn scan_and_insert(
    pool: &DatabaseConnection,
    client: erogamescape_api::Client,
    scan_dirs: Vec<String>,
) -> Result<(), anyhow::Error> {
    let mut tasks_set = JoinSet::new();
    for scan_dir in scan_dirs {
        tasks_set.spawn(async move {
            let dir = tokio::fs::read_dir(&scan_dir).await;
            let mut folders = Vec::new();
            if let Ok(mut dir) = dir {
                while let Ok(Some(dir)) = dir.next_entry().await {
                    if let Ok(ft) = dir.file_type().await {
                        if ft.is_dir() {
                            folders.push(dir.path());
                        }
                    }
                }
            }
            folders
        });
    }

    let mut game_folders = Vec::new();
    while let Some(Ok(mut entries)) = tasks_set.join_next().await {
        game_folders.append(&mut entries)
    }

    let library_registered_games = Game::find()
        .filter(entity::game::Column::LibraryRegistered.eq(true))
        .all(pool)
        .await?;

    let to_unregister_ids = library_registered_games
        .iter()
        .filter_map(|game| {
            if game_folders.iter().any(|folder| match &game.folder {
                Some(f) => f == &folder.to_string_lossy().to_string(),
                None => false,
            }) {
                None
            } else {
                Some(game.id)
            }
        })
        .collect::<Vec<_>>();

    if !to_unregister_ids.is_empty() {
        info!("Unregistering {} items", to_unregister_ids.len());
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("UPDATE game SET library_registered=0 WHERE id IN (");
        let mut separated = query_builder.separated(", ");
        for id in to_unregister_ids {
            separated.push_bind(id);
        }
        separated.push_unseparated(") ");
        let query = query_builder.build();

        query.execute(pool).await?;
    }

    info!("Scanning {} folders", game_folders.len());

    let mut tasks = vec![];
    for game_folder in game_folders {
        let client = client.clone();
        let pool = pool.clone();
        tasks.push(async move {
            let title = game_folder
                .file_name()
                .context("No folder name")?
                .to_str()
                .context("Invalid folder name")?;

            let es_res: Vec<EsGame> = client
                .execute_sql(&format!("SELECT id, brandname, gamename, gyutto_id, dmm, dmm_genre, dmm_genre_2, dlsite_id, dlsite_domain, digiket, comike FROM gamelist WHERE gamename LIKE '{}%' LIMIT 1", title))
                .await?;
            let es_res = es_res.get(0);

            let game_folder = game_folder.to_string_lossy().to_string();

            if let Some(es_res) = es_res {
                debug!("Found folder {} with es id {}, title {}", &game_folder, es_res.id, &es_res.gamename);
                let main_image = if let (Some(dmm), Some(dmm_genre), Some(dmm_genre_2)) = (es_res.dmm.as_ref(), es_res.dmm_genre.as_ref(), es_res.dmm_genre_2.as_ref()) {
                    let img_postfix = if dmm_genre_2 == "doujin" {"r"} else {"l"};
                    Some(format!("https://pics.dmm.co.jp/{dmm_genre}/pcgame/{dmm}/{dmm}p{img_postfix}.jpg")) 
                } else if let (Some(dlsite_domain), Some(dlsite_id)) = (es_res.dlsite_domain.as_ref(), es_res.dlsite_id.as_ref()) {
                    let floor = if dlsite_domain == "pro" || dlsite_domain == "pro2" {
                        "professional"
                    } else {
                        "doujin"
                    };
                    let dlsite_id_to_ceil = &dlsite_id[dlsite_id.len() - 5..];
                    let dlsite_id_to_ceil = dlsite_id_to_ceil.parse::<i32>();
                    if let Ok(dlsite_id_to_ceil) = dlsite_id_to_ceil {
                        let directory_prefix = &dlsite_id[0..2];
                        let directory_id = (((dlsite_id_to_ceil / 10000) as f64).ceil() as i64) * 10000;
                        Some(format!("http://img.dlsite.jp/modpub/images2/work/{floor}/{directory_prefix}{directory_id}/{dlsite_id}"))
                    } else {
                        warn!("Failed to parse dlsite id");
                        None
                    }
                } else {
                    None
                };


                query!("INSERT INTO game 
                       (es_id, name, main_image, library_registered, folder, executable, executable_auto_detect) 
                       VALUES (?, ?, ?, ?, ?, ?, ?)",
                        es_res.id,
                        title,
                        main_image,
                        // es_res.brandname,
                        true,
                        game_folder,
                        None::<Option<String>>,
                        None::<Option<bool>>
                ).execute(&pool).await?;
            } else {
                debug!("Could not found folder {}", &game_folder);
                query!("INSERT INTO game (name, library_registered, folder) VALUES (?, ?, ?)", title, true, game_folder).execute(&pool).await?;
            }

            Ok::<_, anyhow::Error>(())
        });
    }

    let res = futures::future::join_all(tasks).await;

    info!("Scan completed");

    Ok(())
}
