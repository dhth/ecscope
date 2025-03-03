use std::path::PathBuf;

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
    let data_dir = dirs::config_dir().ok_or(ConfigDirError::CouldntGetConfigDir)?;

    Ok(config_dir)
}
