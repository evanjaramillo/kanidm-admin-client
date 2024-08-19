use std::sync::Mutex;

use kanidm_client::KanidmClientConfig;
use serde::{Deserialize, Serialize};

pub type ApplicationState = Mutex<Option<AppState>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    kanidm_config: KanidmClientConfig
}

impl AppState {

}