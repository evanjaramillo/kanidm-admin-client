use std::collections::BTreeMap;

use kanidm_client::{KanidmClientConfig, KanidmClientConfigInstance};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KanidmConfigInstance {
    pub name: Option<String>,
    pub uri: Option<String>,
    pub verify_hostnames: Option<bool>,
    pub verify_ca: Option<bool>,
    pub ca_path: Option<String>
}

impl From<KanidmConfigInstance> for KanidmClientConfigInstance {
    fn from(value: KanidmConfigInstance) -> Self {
        KanidmClientConfigInstance {
            uri: value.uri,
            verify_hostnames: value.verify_hostnames,
            verify_ca: value.verify_ca,
            ca_path: value.ca_path
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KanidmConfig {
    #[serde(flatten)]
    pub default: KanidmConfigInstance,
    #[serde(flatten)]
    pub instances: BTreeMap<String, KanidmConfigInstance>
}

impl From<KanidmConfig> for KanidmClientConfig {
    fn from(value: KanidmConfig) -> Self {
        KanidmClientConfig {
            default: value.default.into(),
            instances: Default::default()
        }
    }
}