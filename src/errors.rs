use crate::cmds::{AddProfileError, ListProfilesError, MonitorError};
use crate::utils::ConfigDirError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    GetConfigDirectory(#[from] ConfigDirError),
    #[error(transparent)]
    AddProfile(#[from] AddProfileError),
    #[error(transparent)]
    ListProfiles(#[from] ListProfilesError),
    #[error(transparent)]
    RunMonitor(#[from] MonitorError),
}

impl AppError {
    pub fn code(&self) -> Option<u16> {
        match self {
            AppError::GetConfigDirectory(e) => match e {
                ConfigDirError::XDGConfigHomeNotAbsolute => None,
                ConfigDirError::CouldntGetConfigDir => Some(100),
            },
            AppError::AddProfile(e) => match e {
                AddProfileError::ProfileNameInvalid(_) => None,
                AddProfileError::CouldntCreateProfileDir(..) => Some(300),
                AddProfileError::ProfileAlreadyExists => None,
                AddProfileError::CouldntOpenFile(_) => Some(301),
                AddProfileError::CouldntWriteToFile(_) => Some(302),
            },
            AppError::ListProfiles(e) => match e {
                ListProfilesError::ReadFilesInDataDir(_) => Some(400),
                ListProfilesError::GetFileFromDataDir(_) => Some(401),
                ListProfilesError::GetFileStem(_) => Some(402),
            },
            AppError::RunMonitor(e) => match e {
                MonitorError::ProfileDoesntExist => None,
                MonitorError::CouldntReadProfileFile(_) => Some(500),
                MonitorError::ConfigFileInvalid(_) => Some(501),
                MonitorError::TUIError(_) => Some(502),
            },
        }
    }
}
