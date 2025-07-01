use crate::aws::get_sdk_config;
use crate::cmds::get_profile_path;
use crate::config::{ClusterConfig, Config, ConfigSource};
use aws_sdk_ecs::Client as ECSClient;
use regex::Regex;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::path::Path;
use std::path::PathBuf;
use toml::de::Error as TomlError;

const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";

#[derive(thiserror::Error, Debug)]
pub enum ConfigDirError {
    #[cfg(target_family = "unix")]
    #[error("{XDG_CONFIG_HOME} is not an absolute path")]
    XDGConfigHomeNotAbsolute,
    #[error("couldn't get your data directory")]
    CouldntGetConfigDir,
}

pub fn get_config_dir() -> Result<PathBuf, ConfigDirError> {
    #[cfg(target_family = "unix")]
    let config_dir = match std::env::var_os(XDG_CONFIG_HOME).map(PathBuf::from) {
        Some(p) => {
            if p.is_absolute() {
                Ok(p)
            } else {
                Err(ConfigDirError::XDGConfigHomeNotAbsolute)
            }
        }
        None => match dirs::config_dir() {
            Some(p) => Ok(p),
            None => Err(ConfigDirError::CouldntGetConfigDir),
        },
    }?;

    #[cfg(not(target_family = "unix"))]
    let config_dir = dirs::config_dir().ok_or(ConfigDirError::CouldntGetConfigDir)?;

    Ok(config_dir)
}

#[derive(thiserror::Error, Debug)]
pub enum GetClustersError {
    #[error("profile doesn't exist")]
    ProfileDoesntExist,
    #[error("couldn't read profile file: {0}")]
    CouldntReadProfileFile(IOError),
    #[error("config file is invalid: {0}")]
    ConfigFileInvalid(#[from] TomlError),
}

pub async fn get_clusters(
    config_dir: &Path,
    profile_name: String,
    service_name_filter: Option<Regex>,
    key_filter: Option<Regex>,
) -> Result<Option<(HashMap<ConfigSource, ECSClient>, Vec<ClusterConfig>)>, GetClustersError> {
    let profile_path = get_profile_path(config_dir, &profile_name);

    if !profile_path.exists() {
        return Err(GetClustersError::ProfileDoesntExist);
    }
    let config_bytes =
        std::fs::read_to_string(profile_path).map_err(GetClustersError::CouldntReadProfileFile)?;
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
        return Ok(None);
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

    Ok(Some((clients_map, clusters)))
}
