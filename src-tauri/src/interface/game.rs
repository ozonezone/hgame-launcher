use chrono::NaiveDateTime;

pub struct Game {
    pub id: i64,
    pub es_id: Option<i64>,
    pub name: String,
    pub main_image: Option<String>,
    pub images: Option<String>,
    pub brand_id: Option<i64>,
    pub library_registered: bool,
    pub library_registered_at: Option<NaiveDateTime>,
    pub last_played_at: Option<NaiveDateTime>,
    pub folder: Option<String>,
    pub executable: Option<String>,
    pub executable_auto_detect: Option<bool>,
    pub play_count: i64,
    pub play_time: i64,
}
