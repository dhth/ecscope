use crate::aws::get_sdk_config;
use crate::cmds::get_profile_path;
use crate::config::{Config, ConfigSource};
use crate::tui::run_tui;
use aws_sdk_ecs::Client as ECSClient;
use regex::Regex;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::path::Path;
use toml::de::Error as TomlError;

#[derive(thiserror::Error, Debug)]
pub enum MonitorError {
    #[error("profile doesn't exist")]
    ProfileDoesntExist,
    #[error("couldn't read profile file: {0}")]
    CouldntReadProfileFile(IOError),
    #[error("config file is invalid: {0}")]
    //ConfigFileInvalid(#[from] SerdeJsonError),
    ConfigFileInvalid(#[from] TomlError),
    #[error(transparent)]
    TUIError(anyhow::Error),
}

pub async fn run_monitor(
    config_dir: &Path,
    profile_name: String,
    service_name_filter: Option<Regex>,
    key_filter: Option<Regex>,
) -> Result<(), MonitorError> {
    let profile_path = get_profile_path(config_dir, &profile_name);

    if !profile_path.exists() {
        return Err(MonitorError::ProfileDoesntExist);
    }
    let config_bytes =
        std::fs::read_to_string(profile_path).map_err(MonitorError::CouldntReadProfileFile)?;
    let app_config: Config = toml::from_str(&config_bytes)?;

    let mut clients_map: HashMap<ConfigSource, ECSClient> = HashMap::new();

    let clusters = match (service_name_filter, key_filter) {
        (None, None) => app_config.clusters,
        (None, Some(k)) => app_config
            .clusters
            .into_iter()
            .filter_map(|c| c.filter_by_cluster_key(&k))
            .collect::<Vec<_>>(),
        (Some(s), None) => app_config
            .clusters
            .into_iter()
            .filter_map(|c| c.filter_by_service_name(&s))
            .collect::<Vec<_>>(),
        (Some(s), Some(k)) => app_config
            .clusters
            .into_iter()
            .filter_map(|c| c.filter_by_service_name(&s))
            .filter_map(|c| c.filter_by_cluster_key(&k))
            .collect::<Vec<_>>(),
    };

    if clusters.is_empty() {
        return Ok(());
    }

    for cluster in &clusters {
        let config = &cluster.config_source;
        match clients_map.get(&cluster.config_source) {
            Some(_) => {}
            None => {
                let sdk_config = get_sdk_config(config).await;
                let client = aws_sdk_ecs::Client::new(&sdk_config);
                clients_map.insert(cluster.config_source.clone(), client);
            }
        }
    }

    run_tui(profile_name, clients_map, clusters)
        .await
        .map_err(MonitorError::TUIError)?;

    Ok(())
}
