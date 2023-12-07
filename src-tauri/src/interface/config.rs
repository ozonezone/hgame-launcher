use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Default, Type)]
pub struct Config {
    pub scan_dir: Vec<String>,
}
