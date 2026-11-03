use std::sync::Arc;

use crate::{config::Config, db::Database};

pub struct AppState {
    pub cfg: Config,
    pub db: Database,
    pub http: reqwest::Client,
}

pub type Shared = Arc<AppState>;
