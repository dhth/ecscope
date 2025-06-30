use super::common::{PROFILE_FILE_EXTENSION, PROFILES_DIR};
use std::path::{Path, PathBuf};

pub fn get_profile_path(config_dir: &Path, profile_name: &str) -> PathBuf {
    config_dir
        .join(PathBuf::from(PROFILES_DIR))
        .join(PathBuf::from(format!(
            "{profile_name}.{PROFILE_FILE_EXTENSION}"
        )))
}
