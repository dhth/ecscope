use super::common::{PROFILE_FILE_EXTENSION, PROFILES_DIR};
use std::io::Error as IOError;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum ListProfilesError {
    #[error("couldn't read files in ecscope's config directory: {0}")]
    ReadFilesInDataDir(IOError),
    #[error("couldn't get file from ecscope's config directory: {0}")]
    GetFileFromDataDir(IOError),
    #[error("couldn't get the name of a file in ecscope's config directory; path: {0}")]
    GetFileStem(String),
}

pub fn list_profiles(config_dir: &Path) -> Result<(), ListProfilesError> {
    let mut profiles = Vec::new();

    let profiles_path = config_dir.join(PathBuf::from(PROFILES_DIR));
    if !profiles_path.exists() {
        return Ok(());
    }

    for entry in std::fs::read_dir(profiles_path).map_err(ListProfilesError::ReadFilesInDataDir)? {
        let entry = entry.map_err(ListProfilesError::GetFileFromDataDir)?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some(PROFILE_FILE_EXTENSION) {
            continue;
        }
        let path_str = path.as_os_str().to_os_string();

        let f = path
            .file_stem()
            .ok_or(ListProfilesError::GetFileStem(
                path.to_string_lossy().to_ascii_lowercase(),
            ))?
            .to_owned();
        profiles.push((f, path_str));
    }

    profiles.sort();

    let output = profiles
        .iter()
        .map(|(stem, path)| {
            format!(
                "{}\t(located at {})",
                stem.to_string_lossy(),
                path.to_string_lossy()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);

    Ok(())
}
