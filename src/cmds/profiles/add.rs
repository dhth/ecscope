use super::common::{PROFILE_FILE_EXTENSION, PROFILES_DIR};
use crate::domain::Profile;
use std::fs::File;
use std::io::Error as IOError;
use std::io::Write;
use std::path::{Path, PathBuf};

const SAMPLE_PROFILE: &[u8] = include_bytes!("static/sample-profile.toml");

#[derive(thiserror::Error, Debug)]
pub enum AddProfileError {
    #[error("profile name is invalid; {0}")]
    ProfileNameInvalid(String),
    #[error("couldn't create profiles directory at {0}: {1}")]
    CouldntCreateProfileDir(PathBuf, IOError),
    #[error("profile already exists")]
    ProfileAlreadyExists,
    #[error("couldn't open file in ecscope's config directory: {0}")]
    CouldntOpenFile(IOError),
    #[error("couldn't write to file in ecscope's data directory: {0}")]
    CouldntWriteToFile(IOError),
}

pub fn add_profile(
    config_dir: &Path,
    profile_name: String,
    overwrite: bool,
) -> Result<(), AddProfileError> {
    let profile =
        Profile::try_from(profile_name.as_str()).map_err(AddProfileError::ProfileNameInvalid)?;

    let profiles_path = config_dir.join(PathBuf::from(PROFILES_DIR));
    if !profiles_path.exists() {
        std::fs::create_dir_all(&profiles_path)
            .map_err(|e| AddProfileError::CouldntCreateProfileDir(profiles_path.clone(), e))?;
    }

    let profile_path = profiles_path.join(PathBuf::from(format!(
        "{}.{}",
        profile.name(),
        PROFILE_FILE_EXTENSION
    )));

    if !overwrite && profile_path.exists() {
        return Err(AddProfileError::ProfileAlreadyExists);
    }

    let mut profile_file = File::create(&profile_path).map_err(AddProfileError::CouldntOpenFile)?;

    profile_file
        .write_all(SAMPLE_PROFILE)
        .map_err(AddProfileError::CouldntWriteToFile)?;

    println!(
        r#"Profile config file added at:
{:?}

You can edit the file in your text editor, and use it via "ecscope -p {}""#,
        &profile_path, &profile_name
    );

    Ok(())
}
