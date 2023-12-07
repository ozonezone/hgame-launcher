mod api;
mod error;

pub use error::Error;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}
