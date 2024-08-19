use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::config::{self, KanidmConfig};

#[derive(Debug)]
pub enum ConfigResolverError {
    Unresolved,
    OpenFailed,
    ReadFailed,
    ParseFailed,
}

pub struct KanidmConfigResolver {
    resolved_path: Option<PathBuf>,
}

pub fn file_exists(path: &Path) -> bool {
    return match path.try_exists() {
        Ok(b) => b,
        Err(e) => {
            log::error!(
                "Error checking {:#?} for existance: {:#?}",
                path, e
            );
            false
        }
    };
}

impl KanidmConfigResolver {
    pub fn new() -> KanidmConfigResolver {

        let sys_config_location = Path::new("/etc/kanidm/config");
        let mut home_config_location = PathBuf::default();

        let home_location = dirs::home_dir();

        if home_location.is_some() {
            // try to resolve the home location:
            home_config_location.push(home_location.unwrap());
            home_config_location.push(".config/kanidm");
        }

        let mut resolved = None;

        if file_exists(home_config_location.as_path()) {
            resolved = Some(home_config_location);
        } else if file_exists(sys_config_location) {
            resolved = Some(sys_config_location.to_path_buf());
        }

        log::debug!("resolved cfg {:#?}", resolved);

        KanidmConfigResolver {
            resolved_path: resolved,
        }
    }

    pub fn is_resolved(&self) -> bool {
        return self.resolved_path.is_some();
    }

    pub fn config_path(&self) -> Option<&Path> {
        self.resolved_path.as_deref()
    }

    pub fn config(&self) -> Result<config::KanidmConfig, ConfigResolverError> {
        if !self.is_resolved() {
            return Err(ConfigResolverError::Unresolved);
        }

        // should be safe to use unwrap here
        let mut config_file = match File::open(self.config_path().unwrap()) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                _ => {
                    return Err(ConfigResolverError::OpenFailed);
                }
            },
        };

        let mut buf = String::new();
        config_file
            .read_to_string(&mut buf)
            .map_err(|_| ConfigResolverError::ReadFailed)?;

        let config: KanidmConfig =
            toml::de::from_str(&buf).map_err(|err| {
                log::error!("Parse error: {:#?}", err);
                ConfigResolverError::ParseFailed
            })?;

        return Ok(config);
    }
}
