// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod config_resolver;
mod error;
mod logging;

use config::KanidmConfig;
use config::KanidmConfigInstance;
use error::Error;
use kanidm_client::KanidmClientBuilder;
use log::debug;
use log::{LevelFilter, SetLoggerError};
use std::sync::Mutex;
use tauri::Manager;
use tauri::Window;

static LOGGER: logging::SimpleLogger = logging::SimpleLogger;

#[tauri::command]
async fn close_login_window(window: Window) {
    // Close splashscreen
    window
        .get_window("login-window")
        .expect("no window labeled 'login-window' found")
        .close()
        .unwrap();

    // Show main window
    window
        .get_window("main")
        .expect("no window labeled 'main' found")
        .show()
        .unwrap();
}

#[tauri::command]
async fn current_config(
    state: tauri::State<'_, Mutex<config::KanidmConfig>>,
) -> Result<config::KanidmConfig, error::Error> {
    let state = state.lock()?;

    Ok(state.clone())
}

#[tauri::command]
async fn fe_logger(lvl: &str, msg: &str) -> Result<(), String> {

    match lvl {
        "trace" => log::trace!("FE: {}", msg),
        "debug" => log::debug!("FE: {}", msg),
        "info" => log::info!("FE: {}", msg),
        "warn" => log::warn!("FE: {}", msg),
        "error" => log::error!("FE: {}", msg),
        _ => {
            log::error!("invalid logging level from front-end: {}", lvl);
            return Err(format!("Invalid level: {}", lvl));
        }
    }

    Ok(())
}

#[tauri::command]
async fn connect(app: tauri::AppHandle, user: &str) -> Result<Vec<String>, error::Error> {
    let mutex = app.state::<Mutex<KanidmConfig>>();

    log::debug!("connecting with user: {:#?}", user);

    let cfg = mutex.lock()?.clone();
    let default_cfg = cfg.default;
    let mut builder = KanidmClientBuilder::new();

    if default_cfg.uri.is_none() {
        return Err(Error::ValueMissing("uri".to_string()));
    }

    builder = builder.address(default_cfg.uri.unwrap());
    builder = builder.danger_accept_invalid_certs(default_cfg.verify_ca.or(Some(false)).unwrap());
    builder = builder
        .danger_accept_invalid_hostnames(default_cfg.verify_hostnames.or(Some(false)).unwrap());

    if default_cfg.ca_path.is_some() {
        builder = builder.add_root_certificate_filepath(default_cfg.ca_path.unwrap().as_str())?;
    }

    builder = builder.request_timeout(10);
    builder = builder.connect_timeout(10);

    let client = builder.build()?;

    if user.is_empty() {
        return Err(Error::ValueMissing("username".to_string()));
    }

    log::info!(
        "attempting initial auth to {:#?}",
        client.get_url().as_str()
    );
    let mechs = match client.auth_step_init(user).await {
        Ok(m) => m,
        Err(e) => {
            log::error!("auth init error: {:#?}", e);
            return Err(Error::from(e));
        }
    };

    let mut mecvec: Vec<String> = Vec::new();

    for value in &mechs {
        mecvec.push(value.to_string());
    }

    Ok(mecvec)
}

fn resolve_default_kanidm_config() -> KanidmConfig {
    let resolver = config_resolver::KanidmConfigResolver::new();

    let config = match resolver.config() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to resolve configuration {:#?}", e);

            KanidmConfig {
                default: KanidmConfigInstance {
                    uri: None,
                    name: None,
                    verify_hostnames: Some(true),
                    verify_ca: Some(true),
                    ca_path: None,
                },
                instances: Default::default(),
            }
        }
    };
    config
}

fn init_logging() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

fn main() {
    let _ = init_logging();
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            handle.manage(Mutex::new(resolve_default_kanidm_config()));

            #[cfg(debug_assertions)]
            {
                debug!("Opening devtools.");
                let window = app.get_window("login-window").unwrap();
                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            close_login_window,
            current_config,
            connect,
            fe_logger
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
